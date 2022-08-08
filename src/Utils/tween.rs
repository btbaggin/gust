use std::ops::{Add, Mul, Sub};

pub struct Tween<T: Copy + Sub<Output = T> + Add<Output = T> + Mul<f32, Output = T>> {
    from: T,
    to: T
}
impl<T: Copy + Sub<Output = T> + Add<Output = T> + Mul<f32, Output = T>> Tween<T> {
    fn new(from: T, to: T) -> Tween<T> {
        Tween { from, to }
    }
    fn lerp(&self, current: T, t: f32) -> T {
        current + (self.to - current) * t
    }

    fn slerp(&self, t: f32) -> T {
        self.from * (1. - t) + self.to * t
    }
}
// pub trait Tween {
//     fn lerp(self, other: Self, t: f32) -> Self;
// }

// impl<T> Tween for T where
//     T: Add<Output = T> + Mul<f32, Output = T> {

//     fn lerp(self, other: T, t: f32) -> T {
//         self * (1. - t) + other * t
//     }
// }