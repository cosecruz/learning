use master_concurrency::{atomic, channels, part_a, thread_pool};

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
    //     part_a::state_chan();
    // part_a::rw();
    // part_a::pk_lot_mutex();
    // atomic::basic();
    // atomic::compare();
    // channels::basic_mpsc();
    // channels::basic_mpsc_recv();
    // channels::worker_pool(3, None);
    // let results = channels::worker_pool_with_results(3, vec![1, 2, 3, 4, 5], |x| x * 2);
    // let results = channels::worker_pool_with_beam_results(3, vec![1, 2, 3, 4, 5], |x| x * 2);

    // println!("results: {:?}", results);

    channels::spmc();
}
