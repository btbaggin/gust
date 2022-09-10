#![allow(dead_code)]
use std::ops::{Add, Mul, Sub};
use std::cmp::PartialEq;

pub struct Tween<T: Copy + Sub<Output = T> + Add<Output = T> + Mul<f32, Output = T> + PartialEq> {
    from: T,
    to: T,
    current: T,
}
impl<T: Copy + Sub<Output = T> + Add<Output = T> + Mul<f32, Output = T> + PartialEq> Tween<T> {
    pub fn new(from: T, to: T) -> Tween<T> {
        Tween { from, to, current: from }
    }
    pub fn lerp(&mut self, t: f32) {
        self.current = self.current + (self.to - self.current) * t;
    }

    pub fn slerp(&mut self, t: f32) {
        self.current = self.from * (1. - t) + self.to * t;
    }
    pub fn set_to(&mut self, to: T) {
        self.to = to;
    }

    pub fn value(&self) -> T {
        self.current
    }

    pub fn is_complete(&self) -> bool {
        self.current == self.to
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