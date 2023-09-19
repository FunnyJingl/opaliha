use std::collections::HashSet;
use std::ops::{Mul, Add, Sub, Neg, Div};
use std::ops::Index;
use num::abs;
use num::Float;
use assert_approx_eq::assert_approx_eq;
use crate::geometry::{point, vector};


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray3 {
    pub origin: point::Point3,
    pub direction: vector::Vector3,
    pub tmax: f64,
    pub time: f64
    // medium
}


impl Ray3 {
    pub fn at(&self, t: f64) -> point::Point3 {
        self.origin + self.direction * t
    }
}
