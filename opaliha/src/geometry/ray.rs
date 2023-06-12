use crate::geometry::point::*;
use crate::geometry::vector::*;

pub struct Ray3 {
    pub p: Point3,
    pub dir: Direction3,
}

impl Default for Ray3 {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            dir: Direction3::default(),
        }
    }
}

// impl Ray3 {
//     pub fn from_vector(vec: Vector3) -> Self {
//         Ray3 {
//             p: vec.p1,
//             dir: vec.dir,
//         }
//     }
// }
