pub mod part_a {
    use std::sync::{Arc, Mutex, mpsc};
    use std::time::Duration;
    use std::{result, thread};

    //basic thread creation
    pub fn basic_thread() {
        let handle = thread::spawn(|| {
            println!("Hello from thread!");
        });

        //wait for thread to finish
        handle.join().unwrap();
    }

    pub fn thread_with_move() {
        let data = vec![1, 2, 3, 4];

        let handle = thread::spawn(move || {
            for i in data {
                println!("{}", i);
            }
        });

        handle.join().unwrap();
    }

    pub fn mult_by_2(n: i32) -> i32 {
        let handle = thread::spawn(move || n * 2);

        handle.join().unwrap()
    }

    pub fn error_hand() {
        let handle = thread::spawn(|| {
            panic!("panicjed in thread");
        });

        match handle.join() {
            Ok(_) => println!("THread succeeeded"),
            Err(e) => eprintln!("Thread panicked: {:?}", e),
        }
    }

    //named threads
    pub fn named() {
        thread::Builder::new()
            .name("Named thread".to_string())
            .stack_size(4 * 1024 * 1024)
            .spawn(|| println!("name: {}", thread::current().name().unwrap()))
            .unwrap()
            .join()
            .unwrap()
    }

    //thread-local-storage

    //send + sync
    //  manual implementation(unsafe)

    struct MyType {
        ptr: *const i32,
    }

    unsafe impl Send for MyType {}

    //Arc + Mutex
    pub fn arc_immut() {
        let counter = Arc::new(0);
        let counter2 = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            println!("in spawned thread; counter: {}", counter2);
        });

        println!("in main thread: {}", counter);

        handle.join().unwrap();
    }

    pub fn arc_mut() {
        let counter = Arc::new(Mutex::new(0));
        let counter2 = Arc::clone(&counter);

        thread::spawn(move || {
            let mut num = counter2.lock().unwrap();
            *num += 1;
        });

        thread::sleep(Duration::from_millis(10));

        println!("{}", counter.lock().unwrap())
    }

    //shared mutable state
    pub fn state() {
        //given a vector of u32; spawn 5 threads to fill the vector from 1 to 100;
        //do locally and partition
        let vector: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));
        let mut handles = vec![];

        for i in 0..5 {
            let data = Arc::clone(&vector);
            let handle = thread::spawn(move || {
                let mut local = vec![];

                let start = i * 20 + 1;
                let end = start + 19;

                for n in start..=end {
                    local.push(n);
                    println!("thread {} pushing {}", i, n)
                }

                data.lock().unwrap().extend(local);
            });

            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap()
        }

        let mut v = vector.lock().unwrap();
        v.sort_unstable();
        println!("{:?}", *v);
    }

    pub fn state_chan() {
        let (tx, rx) = mpsc::channel();

        for i in 0..5 {
            let tx = tx.clone();
            let start = i * 20 + 1;
            let end = start + 19;
            thread::spawn(move || {
                for n in start..=end {
                    tx.send(n).unwrap();
                }
            });
        }
        drop(tx);

        let mut result: Vec<i32> = rx.iter().collect();
        result.sort_unstable();
        println!("{:?}", result);
    }
}

struct TryFn<T> {
    f: fn(&T) -> T, //does not mutate captured values and can be used many times: Fn
    fnmut: fn(&mut T) -> T, // may mutate but not move captured values, callable multiple times: FnMut;
    fnonce: fn(T) -> T,     // may move captured values; only once: FnOnce
}

fn main() {
    // part_a::basic_thread();
    // part_a::thread_with_move();
    // println!("{}", part_a::mult_by_2(10));
    // part_a::error_hand();
    // part_a::named();
    // part_a::arc_immut();
    // part_a::arc_mut();
    // part_a::state();
    part_a::state_chan();
}
