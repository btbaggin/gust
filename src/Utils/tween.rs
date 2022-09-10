#![allow(dead_code)]
use std::ops::{Add, Mul, Sub};
use std::cmp::PartialEq;

pub enum TweenType {
    Lerp,
    Slerp,
}

pub struct Tween {
    from: f32,
    to: f32,
    current: f32,
    amount: f32,
    algorithm: TweenType
}
impl Tween {
    pub fn lerp(from: f32, to: f32) -> Tween {
        Tween { 
            from,
            to,
            current: from,
            amount: 0.,
            algorithm: TweenType::Lerp
        }
    }
    pub fn slerp(from: f32, to: f32) -> Tween {
        Tween { 
            from,
            to,
            current: from,
            amount: 0.,
            algorithm: TweenType::Slerp
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        //TODO this doesnt work
        if self.current >= self.to {
            self.amount += delta_time;
            match self.algorithm {
                TweenType::Lerp => self.current = self.current + (self.to - self.current) * delta_time,
                TweenType::Slerp => self.current = self.from * (1. - self.amount) + self.to * self.amount,
            }
        }
        self.current >= self.to
    }
        
    pub fn set_to(&mut self, to: f32) {
        // self.from = self.to;
        // self.amount = 0.;
        // self.current = self.from;
        self.to = to;
    }

    pub fn value(&self) -> f32 {
        self.current
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