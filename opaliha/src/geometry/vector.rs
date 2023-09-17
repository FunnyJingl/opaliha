use std::ops::{Mul, Add, Sub, Neg};
use std::ops::Index;


#[derive(Debug, PartialEq, Clone, Copy)]
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


impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}


impl Neg for Vector3 {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}


impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}


impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
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
        assert_eq!(v1.len(), 1.);
        assert_eq!(v1.x, 1.);
        assert_eq!(v1.y, 0.);
        assert_eq!(v1.z, 0.);
    }

    #[test]
    fn test_vector_len() {
        let v1 = Vector3::unit_x();
        assert_eq!(v1.len(), 1.);
    }

    #[test]
    fn test_vector_ops() {
        let v1 = Vector3{x: 1., y: 2., z: 3.};
        let v2 = Vector3{x: 4., y: 5., z: 6.};

        let v_sum = v1 + v2;
        assert_eq!(v_sum.x, 5.);
        assert_eq!(v_sum.y, 7.);
        assert_eq!(v_sum.z, 9.);

        let v_neg = -v1;
        assert_eq!(v_neg.x, -1.);
        assert_eq!(v_neg.y, -2.);
        assert_eq!(v_neg.z, -3.);

        let v_sub = v1 - v2;
        assert_eq!(v_sub.x, -3.);
        assert_eq!(v_sub.y, -3.);
        assert_eq!(v_sub.z, -3.);
    }
}
