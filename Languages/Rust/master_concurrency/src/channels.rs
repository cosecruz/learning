use std::sync::{Arc, mpsc};
use std::thread;

use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use parking_lot::Mutex;

//closure pattern
pub fn basic_mpsc() {
    let (tx, rx) = mpsc::channel();
    for n in 0..=2 {
        let tx2 = tx.clone();
        thread::spawn(move || {
            tx2.send(n).unwrap();
        });
    }

    drop(tx);

    let r: Vec<_> = rx.iter().collect();
    println!("{:?}", r)
}

//recieve pattern
pub fn basic_mpsc_recv() {
    let workers = 3;
    let (tx, rx) = mpsc::channel();

    (0..workers).for_each(|n| {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(n).unwrap();
        });
    });

    for _ in 0..workers {
        println!("{}", rx.recv().unwrap());
    }
}

//worker pool (bounded concurrency)

pub fn worker_pool(n: usize, jobs: Option<Vec<i32>>) {
    let jobs = jobs.unwrap_or_else(|| (0..=100).collect());

    //bounded queue
    let (tx, rx) = mpsc::sync_channel::<i32>(n);

    //spawn workers
    spawn_workers_ss(n, rx);

    //submit jobs
    for j in jobs {
        tx.send(j).unwrap();
    }

    drop(tx);
}

fn spawn_workers_ss(workers: usize, rx: mpsc::Receiver<i32>) {
    let rx = Arc::new(Mutex::new(rx));

    for id in 0..workers {
        let rx = Arc::clone(&rx);
        thread::spawn(move || {
            while let Ok(job) = rx.lock().recv() {
                println!("worker {} processing {}", id, job);
            }
            println!("worker {} exiting", id);
        });
    }
}

//worker pool with fan in fan out patterns to collect results

pub fn worker_pool_with_results<T, R, F>(n_workers: usize, jobs: Vec<T>, f: F) -> Vec<R>
where
    T: Send + 'static,
    R: Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static,
{
    //create job channels bounded
    let (job_tx, job_rx) = mpsc::sync_channel::<T>(n_workers);
    //create channels got result
    let (res_tx, res_rx) = mpsc::channel::<R>();

    //spawn workers
    spawn_workers(n_workers, job_rx, res_tx.clone(), f);

    drop(res_tx); //fan_in termination unblocking

    //fan_out: submit jobs
    for job in jobs {
        job_tx.send(job).unwrap();
    }

    drop(job_tx); //signal no more work

    // fan-in: collect results
    res_rx.iter().collect()
}
fn spawn_workers<T, R, F>(
    n_workers: usize,
    job_rx: mpsc::Receiver<T>,
    res_tx: mpsc::Sender<R>,
    f: F,
) where
    T: Send + 'static,
    R: Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static,
{
    let job_rx = Arc::new(Mutex::new(job_rx));
    let f = Arc::new(f);

    for id in 0..n_workers {
        let job_rx = Arc::clone(&job_rx);
        let res_tx = res_tx.clone();
        let f = Arc::clone(&f);

        thread::spawn(move || {
            while let Ok(job) = job_rx.lock().recv() {
                let result = f(job);
                println!("worker {} processed job", id);
                res_tx.send(result).unwrap();
            }
        });
    }
}

//same solution as above but with cross beam
pub fn worker_pool_with_beam_results<T, R, F>(n_workers: usize, jobs: Vec<T>, f: F) -> Vec<R>
where
    T: Send + 'static,
    R: Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static,
{
    let (job_tx, job_rx) = channel::bounded::<T>(n_workers);
    let (res_tx, res_rx) = channel::unbounded::<R>();

    spawn_beam_workers(n_workers, job_rx, res_tx, f);

    // fan-out
    for job in jobs {
        job_tx.send(job).unwrap();
    }
    drop(job_tx); // close job channel

    // fan-in
    res_rx.iter().collect()
}
fn spawn_beam_workers<T, R, F>(n_workers: usize, job_rx: Receiver<T>, res_tx: Sender<R>, f: F)
where
    T: Send + 'static,
    R: Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static,
{
    let f = std::sync::Arc::new(f);

    for id in 0..n_workers {
        let job_rx = job_rx.clone();
        let res_tx = res_tx.clone();
        let f = f.clone();

        thread::spawn(move || {
            while let Ok(job) = job_rx.recv() {
                let result = f(job);
                println!("worker {} processed job", id);
                res_tx.send(result).unwrap();
            }

            println!("worker {} exiting", id);
        });
    }
}

//producer-consumer(streaming)
//single producer and single consumer
pub fn spsc() {
    let (tx, rx) = mpsc::channel::<i32>();

    //producer
    let producer = thread::spawn(move || {
        for n in 0..5 {
            println!("produced {}", n);
            tx.send(n).unwrap();
        }
    });
    //consumer
    let consumer = thread::spawn(move || {
        while let Ok(job) = rx.recv() {
            println!("consumed {}", job);
        }
        println!("consumer done")
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
//single producer and multiple consumer fails
pub fn spmc() {
    let (tx, rx) = channel::unbounded::<i32>();

    // spawn consumers first
    let mut consumers = vec![];
    for i in 0..2 {
        let rx = rx.clone();
        consumers.push(thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                println!("consumer {} got {}", i, msg);
            }
            println!("consumer {} done", i);
        }));
    }

    // producer
    let producer = thread::spawn(move || {
        for n in 1..=5 {
            println!("produced {}", n);
            tx.send(n).unwrap();
        }
        // tx dropped here → channel closes
    });

    producer.join().unwrap();

    // wait for consumers
    for c in consumers {
        c.join().unwrap();
    }
}

//multi producer multi consumer with crossbeam
pub fn mpmc() {
    let (tx, rx) = channel::bounded(5);

    // producers
    for id in 0..2 {
        let tx = tx.clone();
        thread::spawn(move || {
            for n in 0..5 {
                tx.send((id, n)).unwrap();
            }
        });
    }

    // consumers
    for id in 0..3 {
        let rx = rx.clone();
        thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                println!("consumer {} got {:?}", id, msg);
            }
        });
    }

    drop(tx); // close channel
}

//pipelines(multi stage processing)

//shared state
//atomic and message passing(lock free)
//(work stealing)load balancing
//safe cancellation and shutdown- no leaks
//scoped concurrency

//back pressure strategies:
//  bounded channel(block)
//  drop message
// sample/aggregators
// dynamic rate limiting
//  timeouts
//  select

//coordination (barrriers and latches)

//actor model impl
//  basic impl
// actor with handle
//  actor system with supervision

//for what is not written here check obsidian notes
