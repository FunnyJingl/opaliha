use crate::geometry::point;
use crate::geometry::ray;
use crate::geometry::intersection;


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    pub origin: point::Point3,
    pub radius: f64
}


impl Sphere {
    fn new(origin: point::Point3, radius: f64) -> Sphere {
        Sphere { origin, radius }
    }

    pub fn intersect_with_ray(&self, ray: ray::Ray3) -> Option<point::Point3> {
        intersection::intersect_ray_with_sphere(ray, self.origin, self.radius)
    }
}




// #[cfg(test)]
// mod tests {
//     use super::*;
// }
