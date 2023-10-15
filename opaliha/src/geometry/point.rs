use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Mul, Add, Sub};
use std::ops::Index;
use num::abs;
use num::Float;
use crate::geometry::vector::Vector3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}


impl Point3 {
    pub fn origin() -> Point3 {
        Point3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn abs(&self) -> Vector3 {
        Vector3 {x: abs(self.x), y: abs(self.y), z: abs(self.z)}
    }

    pub fn floor(&self) -> Vector3 {
        Vector3 {x: Float::floor(self.x), y: Float::floor(self.y), z: Float::floor(self.z)}
    }

    pub fn ceil(&self) -> Vector3 {
        Vector3 {x: Float::ceil(self.x), y: Float::ceil(self.y), z: Float::ceil(self.z)}
    }

    pub fn permute(&self, xi: usize, yi: usize, zi: usize) -> Point3 {
        if HashSet::from([xi, yi, zi]).len() != 3 { panic!() }
        Point3 {
            x: self[xi],
            y: self[yi],
            z: self[zi]
        }
    }
}


impl Index<usize> for Point3 {
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


impl Add<Vector3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Point3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}


impl Add<Point3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Point3) -> Self::Output {
        Point3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}


impl Sub<Point3> for Point3 {
    type Output = Vector3;

    fn sub(self, rhs: Point3) -> Self::Output {
        Vector3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}


impl Mul<f64> for Point3 {
    type Output = Point3;

    fn mul(self, rhs: f64) -> Self::Output {
        Point3{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}


pub fn distance(p1: Point3, p2: Point3) -> f64 {
    (p2 - p1).norm()
}


pub fn distance_squared(p1: Point3, p2: Point3) -> f64 {
    (p2 - p1).norm().powi(2)
}


pub fn lerp(p1: Point3, p2: Point3, t: f64) -> Point3 {
    p1 * (1. - t) + p2 * t
}


pub fn min_point(p1: Point3, p2: Point3) -> Point3 {
    Point3{
        x: Float::min(p1.x, p2.x),
        y: Float::min(p1.y, p2.y),
        z: Float::min(p1.z, p2.z),
    }
}


pub fn max_point(p1: Point3, p2: Point3) -> Point3 {
    Point3{
        x: Float::max(p1.x, p2.x),
        y: Float::max(p1.y, p2.y),
        z: Float::max(p1.z, p2.z),
    }
}


impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f, "x - {:.5} y - {:.5} z - {:.5}",
            self.x, self.y, self.z).map_err(|err| println!("{:?}", err)).ok();
        Ok(())
    }
}



// #[cfg(test)]
// mod tests {
//     use super::*;
// }
