mod geometry;
// use crate::geometry;
use geometry::point;
mod optical_surfaces;

fn main() {
    let mut point = point::Point3 {
        x: 0.,
        y: 0.,
        z: 1.,
    };

    let mut v = geometry::vector::Vector3::unit_y();
    v.p1.x = 0.1;

    println!("Hello, world!");
}
