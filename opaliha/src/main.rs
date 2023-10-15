mod geometry;
mod optical_surfaces;
mod materials;
use crate::geometry::point::Point3;
use crate::geometry::ray::{Ray3, RayValidity};
use crate::geometry::vector::Vector3;
use crate::optical_system::sequential_optical_system::Trace;

mod optical_system;

fn main() {
    let mut optsys = optical_system::sequential_optical_system::SequentialOpticalSystem::default();

    let surface_0 = optical_system::sequential_optical_system::StandardSurface{
        name: "".to_string(),
        comment: "".to_string(),
        surface_type: optical_system::sequential_optical_system::OpticalSurfaceType::Standard,
        radius: 0.0,
        thickness: 7.,
        material: Box::new(materials::material::Air::default()),
        position: Point3{x: 0., y: 0., z: 0.}
    };

    let surface_1 = optical_system::sequential_optical_system::StandardSurface{
        name: "".to_string(),
        comment: "".to_string(),
        surface_type: optical_system::sequential_optical_system::OpticalSurfaceType::Standard,
        radius: 25.,
        thickness: 7.,
        material: Box::new(materials::material::Air::default()),
        position: Point3{x: 0., y: 0., z: 0.}
    };

    let surface_2 = optical_system::sequential_optical_system::StandardSurface{
        name: "".to_string(),
        comment: "".to_string(),
        surface_type: optical_system::sequential_optical_system::OpticalSurfaceType::Standard,
        radius: -25.,
        thickness: 25.,
        material: Box::new(materials::material::Glass{name: "N-BK7".to_string()}),
        position: Point3{x: 0., y: 0., z: 7.}
    };

    optsys.add_surface(Box::new(surface_1));
    optsys.add_surface(Box::new(surface_2));

    let mut r = Ray3{
        origin: Point3{x: 0., y: 0., z: -50.},
        direction: Vector3{x: 0., y: 5., z: 50.},
        validity: RayValidity::VALID
    };

    r = optsys.trace_ray(r);

    println!("{}", optsys);
    println!("{}", r);

    println!("Done!");
}
