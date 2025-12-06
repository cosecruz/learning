pub fn sum_arr(num_list: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in num_list {
        sum += i;
    }

    sum
}

fn lets_play() {
    let v = vec![1, 2, 3, 4, 5];
    let mut v2: Vec<i32> = Vec::new();

    for i in &v {
        if *i % 2 == 0 {
            v2.push(*i * 2);
        }
    }

    println!("{:?}", v);
}
