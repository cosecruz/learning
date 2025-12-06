use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn mixup<U>(self, other: Point<U>) -> Point<(T, U)> {
        Point {
            x: (self.x, other.x),
            y: (self.y, other.y),
        }
    }
}

impl<T> Point<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Into<f64>,
{
    fn distance_from_origin(&self) -> f64 {
        let x = self.x.into();
        let y = self.y.into();
        (x * x + y * y).sqrt()
    }
}
