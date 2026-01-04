mod closures;
mod concurrency;
mod enums;
mod errors;
mod generics;
mod iterators;
mod maps;
mod slices;
mod smart_pointers;
mod traits;
mod vector;
// fn _vector(){
// let num_list: Vec<i32> = vec![1,2,3];
//   let result = vector::sum_arr(&num_list);

//   println!("{result}");
// }

// fn _enums(){
//   let result = enums::action(TrafficLight::Green);
//   println!("{result}");
// }

use std::env;
use std::path::{Path, PathBuf};

// use crate::maps::learn_maps;
// use crate::slices::first_word;
// use crate::smart_pointers::list;
use crate::concurrency::channels_mpsc;

fn check_path() {
    // Get current working directory
    let mut curr_dir = match env::current_dir() {
        Ok(dir) => dir, // this is a PathBuf
        Err(e) => {
            eprintln!("Failed to get current directory: {e}");
            return; // or handle the error
        }
    };

    // Relative and absolute paths
    //let rel_path = Path::new("./enums.rs").to_str().unwrap(); // relative
    //let abs_path = Path::new("/home/oesisu/work/enums.rs"); // absolute example

    // Using PathBuf to build a path
    //let mut base_path = std::path::PathBuf::from("/src");
    //base_path.push("enum.rs");

    curr_dir.push("test.txt");

    let path = curr_dir.to_str().unwrap();
    println!("{path}");

    // Using match
    match errors::read_number_from_file(path) {
        Ok(number) => println!("Number: {number}"),
        Err(e) => eprintln!("Error: {e}"),
    }

    // Using if let to handle the propagated error
    if let Err(e) = errors::read_number_from_file(path) {
        eprintln!("Error: {e}");
    } else {
        println!("Number read successfully");
    }
}

fn main() {
    // check_path();
    // learn_maps();
    // let s = first_word("A boy");
    // println!("{s}")
    // list();
    // channels();
    channels_mpsc();
    concurrency::run_mutexes()

    // Note: if you want `?` for early return, main must return Result<(), Box<dyn Error>>
}
