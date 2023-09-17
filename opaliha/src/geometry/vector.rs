use crate::point::*;
use std::ops::{Mul, Add};
use std::ops::Index;
use crate::geometry::constants;


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

impl Index<i32> for Vector3 {
    type Output = f64;

    fn index(&self, ind: i32) -> &Self::Output {
        match ind {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexing() {
        let vector = Vector3{x: 1., y: 2., z: 3.};

        assert_eq!(vector[0], 1.);
        assert_eq!(vector[1], 2.);
        assert_eq!(vector[2], 3.);

        let ind3 = std::panic::catch_unwind(|| vector[3]);
        assert!(ind3.is_err())
    }

    #[test]
    fn test_vector_sum() {
        let v1 = Vector3::unit_x();
    }

    #[test]
    fn test_vector_len() {
        let v1 = Vector3::unit_x();
        assert_eq!(v1.len(), 1.);
    }
}
