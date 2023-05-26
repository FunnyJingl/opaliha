use crate::geometry::point::*;
use crate::geometry::vector::*;
use std::ops::Sub;

use super::vector;

impl Sub for Point3 {
    type Output = vector::Vector3;

    fn sub(self, other: Self) -> Vector3 {
        Vector3 {
            p1: self,
            p2: other,
        }
    }
}
