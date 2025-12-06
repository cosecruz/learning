fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    let v = vec![10, 20, 30, 40];
    //let v = vec![a[0], a[1], a[2], a[3]];
    //let v = a.to_vec();

    (a, v)
}

fn main() {
    // You can optionally experiment here.
//     use std::mem::{size_of, align_of};

// println!("size: {}", size_of::<(u8, u32)>());
// println!("align: {}", align_of::<(u8, u32)>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
