use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Mul, Add, Sub, Neg, Div};
use std::ops::Index;
use num::abs;
use num::Float;
use assert_approx_eq::assert_approx_eq;


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}


pub fn zero_vector() -> Vector3 {
    Vector3{x: 0., y: 0., z: 0.}
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

    pub fn abs(&self) -> Vector3 {
        Vector3 {x: abs(self.x), y: abs(self.y), z: abs(self.z)}
    }

    pub fn dot(&self, rhs: Vector3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn abs_dot(&self, rhs: Vector3) -> f64 {
        abs(self.dot(rhs))
    }

    pub fn cross_product(&self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    pub fn norm(&self) -> f64 {
        Float::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }

    pub fn normalize(&mut self) -> &mut Vector3 {
        let norm = self.norm();
        self.x = self.x / norm;
        self.y = self.y / norm;
        self.z = self.z / norm;
        self
    }

    pub fn clone_normalized(&self) -> Vector3 {
        let norm = self.norm();
        Vector3{x: self.x / norm, y: self.y / norm, z: self.z / norm}
    }

    pub fn build_coordinate_system(&self) -> (Vector3, Vector3, Vector3) {
        let mut v1 = self.clone();
        v1 = *v1.normalize();
        let mut v2 = zero_vector();

        if abs(v1.x) > abs(v2.y) {
            let norm_value =  Float::sqrt(v1.x * v1.x + v1.z * v1.z);
            v2.x = -v1.z;
            v2.z = v1.x;
            v2 = v2 / norm_value;
        } else {
            v2.y = v1.z;
            v2.z = -v1.y;
            v2 = v2 / Float::sqrt(v1.y * v1.y + v1.z * v1.z);
        }

        let v3 = v1.cross_product(v2);

        (v1, v2, v3)
    }

    pub fn min_component(&self) -> f64 {
        self.x.min(self.y).min(self.z)
    }

    pub fn max_component(&self) -> f64 {
        self.x.max(self.y).max(self.z)
    }

    pub fn max_dimension(&self) -> usize {
        let v = vec![self.x, self.y, self.z];
        let mut max = v[0];
        let mut min = v[0];
        let mut max_index = 0;
        let mut min_index = 0;
        let mut sum = 0.0;

        for (index, &x) in v.iter().enumerate() {
            if x > max {
                max = x;
                max_index = index;
            }
            if x < min {
                min = x;
                min_index = index;
            }
            sum += x;
        }
        max_index
    }

    pub fn comonent_wise_min(&self, vec: Vector3) -> Vector3 {
        Vector3{
            x: f64::min(self.x, vec.x),
            y: f64::min(self.y, vec.y),
            z: f64::min(self.z, vec.z)}
    }

    pub fn permute(&self, xi: usize, yi: usize, zi: usize) -> Vector3 {
        let mut set = HashSet::from([xi, yi, zi]);
        if set.len() != 3 { panic!() }
        Vector3 {
            x: self[xi],
            y: self[yi],
            z: self[zi]
        }
    }
}


impl Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, ind: usize) -> &Self::Output {
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


impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        match rhs {
            0. => panic!(),
            _ => Vector3{x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
        }
    }
}


impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f, "x - {:.5} y - {:.5} z - {:.5}",
            self.x, self.y, self.z).map_err(|err| println!("{:?}", err)).ok();
        Ok(())
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
    fn test_vector_units() {
        let unit_x = Vector3::unit_x();
        assert_eq!(unit_x.len(), 1.);
        assert_eq!(unit_x.x, 1.);
        assert_eq!(unit_x.y, 0.);
        assert_eq!(unit_x.z, 0.);

        let unit_y = Vector3::unit_y();
        assert_eq!(unit_y.len(), 1.);
        assert_eq!(unit_y.x, 0.);
        assert_eq!(unit_y.y, 1.);
        assert_eq!(unit_y.z, 0.);

        let unit_z = Vector3::unit_z();
        assert_eq!(unit_z.len(), 1.);
        assert_eq!(unit_z.x, 0.);
        assert_eq!(unit_z.y, 0.);
        assert_eq!(unit_z.z, 1.);
    }

    #[test]
    fn test_vector_len() {
        let v1 = Vector3::unit_x();
        assert_eq!(v1.len(), 1.);
    }

    #[test]
    fn test_vector_ops() {
        let v0 = Vector3{x: 0., y: 0., z: 0.};
        let v1 = Vector3{x: 1., y: 2., z: 3.};
        let v2 = Vector3{x: 4., y: 5., z: 6.};
        let v3 = Vector3{x: -7., y: -8., z: -9.};

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

        let v_div = v1 / 10.;
        assert_eq!(v_div.x, 0.1);
        assert_eq!(v_div.y, 0.2);
        assert_eq!(v_div.z, 0.3);

        let v_abs = v3.abs();
        assert_eq!(v_abs.x, 7.);
        assert_eq!(v_abs.y, 8.);
        assert_eq!(v_abs.z, 9.);

        assert_eq!(v0.min_component(), 0.);
        assert_eq!(v1.min_component(), 1.);
        assert_eq!(v2.min_component(), 4.);
        assert_eq!(v3.min_component(), -9.);

        assert_eq!(v0.max_dimension(), 0);
        assert_eq!(v1.max_dimension(), 2);
        assert_eq!(v2.max_dimension(), 2);
        assert_eq!(v3.max_dimension(), 0);

        assert_eq!(v0.comonent_wise_min(v1), Vector3{x: 0., y: 0., z: 0.});
        assert_eq!(v1.comonent_wise_min(v2), Vector3{x: 1., y: 2., z: 3.});
        assert_eq!(v2.comonent_wise_min(v3), Vector3{x: -7., y: -8., z: -9.});

        let v0_normalized = v0.clone().clone_normalized();
        let v1_normalized = v1.clone().clone_normalized();
        let v2_normalized = v2.clone().clone_normalized();
        let v3_normalized = v3.clone().clone_normalized();
        assert!(v0_normalized.norm().is_nan());
        assert_approx_eq!(v1_normalized.norm(), 1.);
        assert_approx_eq!(v2_normalized.norm(), 1.);
        assert_approx_eq!(v3_normalized.norm(), 1.);

        // todo complete tests
        let v0_permute = v0.permute(2, 1, 0);
        assert_eq!(v0_permute, v0);
        assert!(std::panic::catch_unwind(|| v0.permute(0, 1, 0)).is_err());
        assert!(std::panic::catch_unwind(|| v0.permute(0, 0, 0)).is_err());

        let v1_normalized = v1.clone().clone_normalized();
        let v2_normalized = v2.clone().clone_normalized();
        let v3_normalized = v3.clone().clone_normalized();
        assert!(v0_normalized.norm().is_nan());
        assert_approx_eq!(v1_normalized.norm(), 1.);
        assert_approx_eq!(v2_normalized.norm(), 1.);
        assert_approx_eq!(v3_normalized.norm(), 1.);

        let v0_norm = v0.norm();
        let v1_norm = v1.norm();
        let v2_norm = v2.norm();
        let v3_norm = v3.norm();
        assert_approx_eq!(v0_norm, 0.);
        assert_approx_eq!(v1_norm, 3.7416573867739413);
        assert_approx_eq!(v2_norm, 8.774964387392123);
        assert_approx_eq!(v3_norm, 13.92838827718412);

        let dot_prod_0_1 = v0.dot(v1);
        let dot_prod_1_2 = v1.dot(v2);
        let dot_prod_2_3 = v2.dot(v3);
        let dot_prod_3_1 = v3.dot(v1);
        assert_eq!(dot_prod_0_1, 0.);
        assert_eq!(dot_prod_1_2, 32.);
        assert_eq!(dot_prod_2_3, -122.);
        assert_eq!(dot_prod_3_1, -50.);

        let dot_prod_0_1 = v0.abs_dot(v1);
        let dot_prod_1_2 = v1.abs_dot(v2);
        let dot_prod_2_3 = v2.abs_dot(v3);
        let dot_prod_3_1 = v3.abs_dot(v1);
        assert_eq!(dot_prod_0_1, 0.);
        assert_eq!(dot_prod_1_2, 32.);
        assert_eq!(dot_prod_2_3, 122.);
        assert_eq!(dot_prod_3_1, 50.);

        let cross_prod_0_1 = v0.cross_product(v1);
        let cross_prod_1_2 = v1.cross_product(v2);
        let cross_prod_2_3 = v2.cross_product(v3);
        let cross_prod_3_1 = v3.cross_product(v1);
        assert_eq!(cross_prod_0_1, Vector3{x:0., y:0., z:0.});
        assert_eq!(cross_prod_1_2, Vector3{x: -3., y: 6., z: -3.});
        assert_eq!(cross_prod_2_3, Vector3{x: 3., y: -6., z: 3.});
        assert_eq!(cross_prod_3_1, Vector3{x: -6., y: 12., z: -6.});

        // todo complete tests
        let vec_sys = v1.build_coordinate_system();
    }
}
