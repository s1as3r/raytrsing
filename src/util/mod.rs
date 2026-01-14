pub mod rand;
use std::{f64::consts::PI, ops};

#[inline]
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

#[inline]
pub fn lerp<T>(start: T, end: T, t: f64) -> T
where
    T: ops::Mul<f64, Output = T> + ops::Add<Output = T>,
{
    start * (1.0 - t) + end * t
}
