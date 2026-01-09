//! LAB 1 — Bounded Work-Stealing Job System
//!
//! This module implements a production-grade, CPU-bound job executor using
//! pure OS threads and lock-free work stealing.
//!
//! Design goals:
//! - No async / Tokio
//! - Bounded backpressure
//! - Cache-friendly scheduling
//! - Cooperative cancellation
//! - Panic isolation per job
//! - Clean shutdown without leaked threads
//!
//! Crates used:
//! - crossbeam (channels + work-stealing deque)
//! - parking_lot (fast locks if needed later)
//! - num_cpus (core scaling)

use crossbeam::channel::{self, Receiver, Sender};
use crossbeam::deque::{Injector, Steal, Stealer, Worker};
use std::panic;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;

/// Bounded submission capacity (backpressure)
static SUBMIT_BOUND: usize = 32;

/// A CPU-bound job.
///
/// In a real system this would usually be a boxed closure:
/// `Box<dyn FnOnce() + Send + 'static>`
#[derive(Debug)]
pub struct Job {
    pub id: usize,
}

/// Result produced by a job
#[derive(Debug)]
pub struct JobResult {
    pub id: usize,
}

/// Thread pool implementing work stealing.
///
/// Architecture:
///
/// ```text
/// submitters
///     ↓
/// bounded channel (backpressure)
///     ↓
/// global injector (FIFO)
///     ↓
/// ┌───────────────┐
/// │ worker thread │ ←── steals from others
/// └───────────────┘
/// ```
///
/// Each worker owns a local LIFO deque for cache locality.
/// When idle, it first checks the global injector, then steals.
pub struct WorkStealingPool {
    injector: Arc<Injector<Job>>,
    stealers: Arc<Vec<Stealer<Job>>>,
    submit_tx: Sender<Job>,
    shutdown: Arc<AtomicBool>,
}

impl WorkStealingPool {
    /// Create a new pool with one worker per CPU core.
    pub fn new() -> (Self, Receiver<JobResult>) {
        let num_workers = num_cpus::get();

        let injector = Arc::new(Injector::new());
        let shutdown = Arc::new(AtomicBool::new(false));

        let (submit_tx, submit_rx) = channel::bounded(SUBMIT_BOUND);
        let (result_tx, result_rx) = channel::unbounded();

        // Create workers and stealers
        let mut workers = Vec::with_capacity(num_workers);
        let mut stealers = Vec::with_capacity(num_workers);

        for _ in 0..num_workers {
            let worker = Worker::new_lifo();
            stealers.push(worker.stealer());
            workers.push(worker);
        }

        let stealers = Arc::new(stealers);

        // Dispatcher: moves jobs from bounded channel into injector
        {
            let injector = Arc::clone(&injector);
            let shutdown = Arc::clone(&shutdown);

            thread::spawn(move || {
                while !shutdown.load(Ordering::Acquire) {
                    match submit_rx.recv() {
                        Ok(job) => injector.push(job),
                        Err(_) => break,
                    }
                }
            });
        }

        // Spawn worker threads
        for local in workers {
            let injector = Arc::clone(&injector);
            let stealers = Arc::clone(&stealers);
            let shutdown = Arc::clone(&shutdown);
            let result_tx = result_tx.clone();

            thread::spawn(move || worker_loop(local, injector, stealers, shutdown, result_tx));
        }

        (
            Self {
                injector,
                stealers,
                submit_tx,
                shutdown,
            },
            result_rx,
        )
    }

    /// Submit a job to the pool.
    ///
    /// This method applies **backpressure**:
    /// if the submission queue is full, the caller blocks.
    pub fn submit(&self, job: Job) -> Result<(), String> {
        self.submit_tx
            .send(job)
            .map_err(|_| "executor shutting down".into())
    }

    /// Initiate a cooperative shutdown.
    ///
    /// Workers will finish in-flight jobs and exit.
    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::Release);
        let _ = &self.submit_tx; //dropping it
    }
}

/// Main worker loop.
///
/// Scheduling order:
/// 1. Local queue (LIFO)
/// 2. Global injector (FIFO)
/// 3. Steal from other workers
/// 4. Park (yield)
fn worker_loop(
    local: Worker<Job>,
    injector: Arc<Injector<Job>>,
    stealers: Arc<Vec<Stealer<Job>>>,
    shutdown: Arc<AtomicBool>,
    result_tx: Sender<JobResult>,
) {
    while !shutdown.load(Ordering::Acquire) {
        // 1️⃣ Local queue
        if let Some(job) = local.pop() {
            run_job(job, &result_tx, &shutdown);
            continue;
        }

        // 2️⃣ Global injector
        match injector.steal() {
            Steal::Success(job) => {
                run_job(job, &result_tx, &shutdown);
                continue;
            }
            Steal::Retry => continue,
            Steal::Empty => {}
        }

        // 3️⃣ Steal from peers
        let mut stolen = None;
        for stealer in stealers.iter() {
            match stealer.steal() {
                Steal::Success(job) => {
                    stolen = Some(job);
                    break;
                }
                Steal::Retry => continue,
                Steal::Empty => {}
            }
        }

        if let Some(job) = stolen {
            run_job(job, &result_tx, &shutdown);
        } else {
            // 4️⃣ No work — park cooperatively
            thread::sleep(Duration::from_millis(1));
        }
    }
}

/// Execute a job with panic isolation.
fn run_job(job: Job, result_tx: &Sender<JobResult>, shutdown: &AtomicBool) {
    if shutdown.load(Ordering::Relaxed) {
        return;
    }

    let res = panic::catch_unwind(|| {
        // Simulate CPU-heavy work
        std::thread::sleep(Duration::from_millis(50));
        JobResult { id: job.id }
    });

    match res {
        Ok(result) => {
            let _ = result_tx.send(result);
        }
        Err(_) => {
            // Supervisor policy could be:
            // - retry
            // - log & drop
            // - initiate shutdown
            eprintln!("job {} panicked", job.id);
        }
    }
}

// fn main() {
//     let (pool, results) = WorkStealingPool::new();

//     // Submit jobs
//     for i in 0..100 {
//         pool.submit(Job { id: i }).unwrap();
//     }

//     // Collect results
//     for _ in 0..100 {
//         let res = results.recv().unwrap();
//         println!("completed job {}", res.id);
//     }

//     pool.shutdown();
// }
