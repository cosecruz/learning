use master_concurrency::{part_a, thread_pool};

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
    part_a::pk_lot_mutex();
}
