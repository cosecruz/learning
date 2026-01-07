pub mod raw_fifo {
    // use std::collections::VecDeque;
    // use std::sync::{Arc, Condvar, Mutex};
    // use std::thread;

    // type Job = Box<dyn FnOnce() + Send + 'static>;

    // struct ThreadPool {
    //     queue: Arc<(Mutex<VecDeque<Job>>, Condvar)>,
    // }

    // impl ThreadPool {
    //     fn new(n: usize) -> Self {
    //         let queue = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));

    //         for _ in 0..n {
    //             let q = Arc::clone(&queue);
    //             thread::spawn(move || {
    //                 loop {
    //                     let job = {
    //                         let (lock, cvar) = &*q;
    //                         let mut queue = lock.lock().unwrap();

    //                         while queue.is_empty() {
    //                             queue = cvar.wait(queue).unwrap();
    //                         }

    //                         queue.pop_front()
    //                     };

    //                     if let Some(job) = job {
    //                         job();
    //                     }
    //                 }
    //             });
    //         }
    //         Self { queue }
    //     }

    //     fn execute<F>(&self, f: F)
    //     where
    //         F: FnOnce() + Send + 'static,
    //     {
    //         let (lock, cvar) = &*self.queue;
    //         lock.lock().unwrap().push_back(Box::new(f));
    //         cvar.notify_one();
    //     }
    // }
}

use rayon::prelude::*;

pub fn ray_pool() {
    let data = vec![1, 2, 3, 4, 5, 6];

    let doubles: Vec<_> = data.par_iter().map(|&n| n * 2).collect();

    println!("{:?}", doubles);
}
