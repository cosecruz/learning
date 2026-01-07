use std::sync::atomic::{AtomicUsize, Ordering};

pub fn basic() {
    let x = AtomicUsize::new(0);

    x.store(42, Ordering::SeqCst);

    let value = x.load(Ordering::SeqCst);
    println!("{}", value);

    let old = x.fetch_add(5, Ordering::SeqCst);
    println!("old: {}, new: {}", old, x.load(Ordering::SeqCst));
    // old: 10, new: 15

    // Other fetch operations:
    x.fetch_sub(3, Ordering::SeqCst); // Subtract
    x.fetch_and(0xFF, Ordering::SeqCst); // Bitwise AND
    x.fetch_or(0x10, Ordering::SeqCst); // Bitwise OR
    x.fetch_xor(0x01, Ordering::SeqCst); // Bitwise XOR
    x.fetch_max(20, Ordering::SeqCst); // Max
    x.fetch_min(5, Ordering::SeqCst); // Min
}

pub fn compare() {
    let x = AtomicUsize::new(10);

    let result = x.compare_exchange(10, 20, Ordering::SeqCst, Ordering::SeqCst);

    match result {
        Ok(10) => println!("success"),
        Ok(_) => unreachable!("compare_exchange only succeeds with expected value"),
        Err(actual) => eprintln!("failed actual val: {}", actual),
    }
}

fn stati() {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    {
        //thread 1
        COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    {
        //thread 2
        COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    if COUNTER.load(Ordering::SeqCst) == 2 {
        println!("success")
    }
}
