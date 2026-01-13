// write below in Future
// async fn fetch_data() -> String {
//     let conn = connect().await; // Suspension point 1
//     let data = conn.read().await; // Suspension point 2
//     data
// }

// use std::task::Poll;

// enum FutureFetchData {
//     Start,
//     Connecting(String),
//     Reading(String),
//     Done(String),
// }

// impl Future for FutureFetchData {
//     type Output = String;
//     fn poll(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Self::Output> {
//         let this = self.get_mut();

//         match this {
//             FutureFetchData::Start => {
//                 *this = FutureFetchData::Connecting("connecting".into());
//                 Poll::Pending
//             }
//             FutureFetchData::Connecting(s) => {
//                 *this = FutureFetchData::Reading(s.clone());
//                 Poll::Pending
//             }

//             FutureFetchData::Reading(s) => {
//                 *this = FutureFetchData::Done(s.clone());
//                 Poll::Pending
//             }

//             FutureFetchData::Done(result) => Poll::Ready(result.clone()),
//         }
//     }
// }

use std::{
    future::Future,
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

enum FetchState {
    Start,
    Connecting,
    Reading,
    Done,
}

pub struct FetchFuture {
    state: FetchState,
    ready: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl FetchFuture {
    pub fn new() -> Self {
        let ready = Arc::new(AtomicBool::new(false));
        let waker: Arc<Mutex<Option<Waker>>> = Arc::new(Mutex::new(None));

        // Simulated I/O event (like epoll waking us)
        {
            let ready = ready.clone();
            let waker = waker.clone();

            thread::spawn(move || {
                thread::sleep(Duration::from_secs(1));
                ready.store(true, Ordering::Release);

                if let Some(w) = waker.lock().unwrap().take() {
                    w.wake();
                }
            });
        }

        Self {
            state: FetchState::Start,
            ready,
            waker,
        }
    }
}

impl Future for FetchFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        loop {
            match this.state {
                FetchState::Start => {
                    println!("[future] start");
                    this.state = FetchState::Connecting;
                }

                FetchState::Connecting => {
                    println!("[future] connecting…");

                    if this.ready.load(Ordering::Acquire) {
                        this.state = FetchState::Reading;
                    } else {
                        // Register waker and return
                        *this.waker.lock().unwrap() = Some(cx.waker().clone());
                        return Poll::Pending;
                    }
                }

                FetchState::Reading => {
                    println!("[future] reading data");
                    this.state = FetchState::Done;
                }

                FetchState::Done => {
                    println!("[future] done");
                    return Poll::Ready("FETCH COMPLETE".into());
                }
            }
        }
    }
}
