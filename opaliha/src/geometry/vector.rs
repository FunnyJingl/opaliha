use crate::point::*;
use std::ops::{Mul, Add};


#[derive(Debug, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}


impl Vector3 {
    pub fn unit_x() -> Vector3 {
        Vector3 {
            x: 1.,
            y: 0.,
            z: 0.,
        }
    }
    pub fn unit_y() -> Vector3 {
        Vector3 {
            x: 0.,
            y: 1.,
            z: 0.,
        }
    }
    pub fn unit_z() -> Vector3 {
        Vector3 {
            x: 0.,
            y: 0.,
            z: 1.,
        }
    }
    pub fn len(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vector_sum() {
        let v1 = Vector3::unit_x();
        // assert_eq!(, );
    }

    #[test]
    fn test_vector_len() {
        let v1 = Vector3::unit_x();
        assert_eq!(v1.len(), 1.);
    }
}
