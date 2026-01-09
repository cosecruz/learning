use crossbeam::channel::{self, Receiver, Sender};
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::panic;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;

static SUBMIT_BOUND: usize = 32;

#[derive(Debug)]
pub struct Job {
    pub id: usize,
}

#[derive(Debug)]
pub struct JobResult {
    pub id: usize,
}

pub struct RayonExecutor {
    submit_tx: Sender<Job>,
    shutdown: Arc<AtomicBool>,
}

impl RayonExecutor {
    pub fn new() -> (Self, Receiver<JobResult>) {
        let shutdown = Arc::new(AtomicBool::new(false));

        let (submit_tx, submit_rx) = channel::bounded::<Job>(SUBMIT_BOUND);
        let (result_tx, result_rx) = channel::unbounded::<JobResult>();

        let pool = ThreadPoolBuilder::new()
            .num_threads(num_cpus::get())
            .thread_name(|i| format!("rayon-worker-{i}"))
            .build()
            .expect("failed to build rayon pool");

        {
            let shutdown = Arc::clone(&shutdown);
            let result_tx = result_tx.clone();

            thread::spawn(move || {
                dispatcher_loop(submit_rx, pool, shutdown, result_tx);
            });
        }

        (
            Self {
                submit_tx,
                shutdown,
            },
            result_rx,
        )
    }

    pub fn submit(&self, job: Job) -> Result<(), String> {
        self.submit_tx
            .send(job)
            .map_err(|_| "executor shutting down".to_string())
    }

    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::Release);
        drop(&self.submit_tx);
    }
}

fn dispatcher_loop(
    submit_rx: Receiver<Job>,
    pool: ThreadPool,
    shutdown: Arc<AtomicBool>,
    result_tx: Sender<JobResult>,
) {
    while !shutdown.load(Ordering::Acquire) {
        let job = match submit_rx.recv() {
            Ok(job) => job,
            Err(_) => break,
        };

        let shutdown = Arc::clone(&shutdown);
        let result_tx = result_tx.clone();

        pool.spawn(move || {
            run_job(job, shutdown, result_tx);
        });
    }
}

fn run_job(job: Job, shutdown: Arc<AtomicBool>, result_tx: Sender<JobResult>) {
    if shutdown.load(Ordering::Relaxed) {
        return;
    }

    let result = panic::catch_unwind(|| {
        // simulate CPU-bound work
        thread::sleep(Duration::from_millis(50));
        JobResult { id: job.id }
    });

    match result {
        Ok(res) => {
            let _ = result_tx.send(res);
        }
        Err(_) => {
            eprintln!("job {} panicked", job.id);
        }
    }
}

// fn main() {
//     let (executor, results) = RayonExecutor::new();

//     for i in 0..100 {
//         executor.submit(Job { id: i }).unwrap();
//     }

//     for _ in 0..100 {
//         let res = results.recv().unwrap();
//         println!("completed job {}", res.id);
//     }

//     executor.shutdown();
// }
