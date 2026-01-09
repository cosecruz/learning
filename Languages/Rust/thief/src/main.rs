use std::thread::{self, sleep};

use crossbeam::channel::{self, Receiver, Sender};

struct Job<T: Send + 'static> {
    data: Vec<T>,
}
trait Executor {
    type Output;
    fn execute(self) -> Result<Self::Output, String>;
}

static N_BOUND: usize = 3;
impl<T: Send + 'static> Executor for Job<T> {
    type Output = Vec<T>;

    fn execute(self) -> Result<Self::Output, String> {
        let (job_tx, job_rx) = channel::bounded(N_BOUND);
        let (res_tx, res_rx) = channel::bounded(N_BOUND);

        Self::spawn_workers(N_BOUND, job_rx.clone(), res_tx.clone());

        drop(res_tx);

        for job in self.data {
            job_tx.send(job).unwrap()
        }

        drop(job_tx);

        todo!();
    }
}

impl<T> Job<T>
where
    T: Send + 'static,
{
    fn spawn_workers(n: usize, job_rx: Receiver<T>, res_tx: Sender<T>) {
        //what is sent can either be an error or data
        //cancellation and error in case job bugs
        //i need work stealing
        //

        for i in 0..n {
            let job_rx = job_rx.clone();
            let res_tx = res_tx.clone();
            thread::spawn(move || {
                while let Ok(job) = job_rx.recv() {
                    //simulate job
                    res_tx.send(job).unwrap();
                }
            });
        }
    }
}

fn main() {
    println!("Hello, world!");
}
