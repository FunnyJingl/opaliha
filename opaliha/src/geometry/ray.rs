use std::fmt;
use crate::geometry::{point, sphere};
use crate::geometry::intersection::intersect_ray_with_sphere;
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
        self.origin = point::Point3{
            x: self.origin.x + self.direction.x * t,
            y: self.origin.y + self.direction.y * t,
            z: self.origin.z + self.direction.z * t,
        };
        true
    }

    pub fn intersect_with_sphere(&self, sphere: sphere::Sphere) -> Option<point::Point3>{
        intersect_ray_with_sphere(*self, sphere.origin, sphere.radius)
    }
}


impl fmt::Display for Ray3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "o - {}, dir - {}, validity - {}",
            self.origin, self.direction, self.validity).map_err(|err| println!("{:?}", err)).ok();
        Ok(())
    }
}

impl fmt::Display for RayValidity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RayValidity::VALID => write!(f, "valid"),
            RayValidity::INVALID => write!(f, "invalid"),
            RayValidity::TIR => write!(f, "tir"),
        }
    }
}
