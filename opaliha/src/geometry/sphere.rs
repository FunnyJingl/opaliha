use super::point::Point3;

struct Sphere {
    center: Point3,
}

impl Sphere {
    pub fn new(center: Point3) -> Self {
        Self { center }
    }
}
