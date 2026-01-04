use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard, mpsc};
use std::thread;
use std::time::Duration;

pub fn channels_mpsc() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();
    thread::spawn(move || {
        let h = String::from("hello");
        tx2.clone().send(h).unwrap();
        thread::sleep(Duration::from_millis(1));
    });

    thread::spawn(move || {
        let h = String::from("world");
        tx.send(h).unwrap();
        thread::sleep(Duration::from_millis(1));
    });

    for received in rx {
        println!("{}", received);
    }
}

pub fn channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let recvd = rx.recv().unwrap();
    println!("{recvd}")
}

pub fn run_mutexes() {
    run_thm();
}

fn run_thm() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
