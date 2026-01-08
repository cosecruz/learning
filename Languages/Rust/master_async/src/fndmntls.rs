use std::sync::{Arc, Mutex};
use std::time::Duration;
struct ReadyFuture {
    value: i32,
}

impl Future for ReadyFuture {
    type Output = i32;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::task::Poll::Ready(self.value)
    }
}

pub async fn use_ready_fut() {
    let val = ReadyFuture { value: 42 }.await;
    println!("{}", val);
}

//task and concurrency
pub async fn do_task() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));

    let mut handles = vec![];

    (0..2).for_each(|n| {
        let data = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let mut g = data.lock().unwrap();
            g.push(n + 6)
        });

        handles.push(handle);
    });

    for h in handles {
        h.await.unwrap();
    }

    let res = data.lock().unwrap();
    println!("{:?}", *res);
}

//select
//  cancellation
//  timeouts
//  supervisors
//   backpressure
//  coordination

//timers
async fn timed_job() {
    let jobs = vec![1, 2, 3, 4];

    //let the jib finish under n milli_sec or return timeout signal
    tokio::time::timeout(Duration::from_millis(20), job()).await;
}

async fn job() {}
//stream
//channel -> stream
// async fn channel_stream() {
//     let (tx, rs) = tokio::sync::mpsc::channel(10);

//     tokio::spawn(async move {
//         for i in 0..5 {
//             tx.send(i).await.unwrap();
//         }
//     });

//     let mut stream = Receiver::
// }
