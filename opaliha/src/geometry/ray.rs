use crate::geometry::point;
use crate::geometry::point::Point3;
use crate::geometry::vector::Vector3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RayValidity {
    VALID,
    INVALID,
    TIR,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray3 {
    pub origin: point::Point3,
    pub direction: Vector3,
    pub validity: RayValidity
}


impl Ray3 {
    pub fn at(&self, t: f64) -> point::Point3 {
        self.origin + self.direction * t
    }

    fn new(p: point::Point3, v: Vector3) -> Ray3 {
        Ray3{origin: p, direction: v.clone_normalized(), validity: RayValidity::VALID}
    }

    pub fn propagate_to_z(&mut self, z: f64) -> bool {
        if (self.origin.z >= z) ||
            (self.direction.z <= 0.) ||
            (self.validity == RayValidity::INVALID) {
            return false
        }
        let dot_prod = self.direction.dot(Vector3::unit_z());
        if dot_prod == 0. {
            return false
        }
        // let t = sum([(a-b)*c for a,b,c in zip(plane_pt, line_pt, plane_norm)]) / dot_prod
        let t = (z - self.origin.z) / dot_prod;
        self.origin = Point3{
            x: self.origin.x + self.direction.x * t,
            y: self.origin.y + self.direction.y * t,
            z: self.origin.z + self.direction.z * t,
        };
        true
    }
}
