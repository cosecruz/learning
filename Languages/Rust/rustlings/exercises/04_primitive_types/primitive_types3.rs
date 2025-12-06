fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
     let a: [i32; 100] = [0;100];

     let _b: [i32; 100] = std::array::from_fn(|i| (i +1) as i32);
     let _v: Vec<i32> = (0..100).collect();


    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
