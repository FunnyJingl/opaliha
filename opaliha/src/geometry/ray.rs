use crate::geometry::point::*;

pub struct Ray3 {
    pub p: Point3,
}

impl Default for Ray3 {
    fn default() -> Self {
        Self {
            p: Point3::default(),
        }
    }
}
