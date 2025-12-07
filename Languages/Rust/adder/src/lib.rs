use std::ops::Add;

//fn add collects an valid number
pub fn add_manual<T>(n1: T, n2: T) -> T
where
    T: Add<Output = T>,
{
    n1 + n2
}

pub fn add_num_traits<T>(n1: T, n2: T) -> T
where
    T: num_traits::Num,
{
    n1 + n2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_manual(1, 2);

        assert_eq!(result, 3);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    #[test]
    #[should_panic]
    fn show_panic() {
        panic!("panic attack");
    }
}
