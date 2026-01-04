use std::thread;
use std::time::Duration;

pub fn cl() {
    // fn add_one_v1(x: u32) -> u32 {
    //     x + 1
    // }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| x + 1;
    // let add_one_v4 = |x| x + 1;
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
}

fn apply<F>(func: F)
where
    F: Fn(i32) -> i32,
{
    println!("{}", func(10));
}
