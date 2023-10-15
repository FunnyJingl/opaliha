use std::fmt::Formatter;
use std::fmt;
use crate::geometry::point::Point3;
use crate::materials;
use crate::geometry::ray::Ray3;
use crate::geometry::sphere;

pub enum OpticalSurfaceType {
    // Biconic,
    // BiconicZernike,
    // ChebyshevPolynomial,
    // EvenAsphere,
    // ExtendedAsphere,
    // ExtendedOddAsphere,
    // ExtendedPolynomial,
    // GridSag,
    // Irregular,
    // OddAsphere,
    // OddCosine,
    // OffAxisConicFreeform,
    // Periodic,
    // Polynomial,
    // QTypeAsphere,
    // QTypeFreeform,
    Standard,
    // Superconic,
    // Tilted,
    // Toroidal,
    // ZernikeFringeSag,
    // ZernikeStandardSag,
    // ZernikeAnnularStandardSag
}

impl fmt::Display for OpticalSurfaceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpticalSurfaceType::Standard => write!(f, "Standard"),
        }
    }
}

impl Default for OpticalSurfaceType {
    fn default() -> Self {
        OpticalSurfaceType::Standard
    }
}

pub trait OpticalSurface : fmt::Debug {
    fn name(&self) -> &str;
    fn comment(&self) -> &str;
    fn surface_type(&self) -> &OpticalSurfaceType;
    fn radius(&self) -> Option<f64>;
    fn thickness(&self) -> Option<f64>;
    fn position(&self) -> Point3;
    fn trace(&self, ray: Ray3) -> Option<Ray3>;
}

pub struct StandardSurface {
    pub name: String,
    pub comment: String,
    pub surface_type: OpticalSurfaceType,
    pub radius: f64,
    pub thickness: f64,
    pub material: Box<dyn materials::material::Material>,
    pub position: Point3,
}


impl StandardSurface {
    pub fn as_sphere(&self) -> Option<sphere::Sphere> {
        if self.radius <= 0.0 { return None }
        Some(sphere::Sphere{origin: self.positio, radius: self.radius})
    }
}

pub trait Trace {
    fn trace_ray(&self, ray: Ray3) -> Ray3;
}

impl fmt::Debug for StandardSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "pars")
    }
}

impl OpticalSurface for StandardSurface {
    fn name(&self) -> &str {
        &self.name
    }
    fn comment(&self) -> &str {
        &self.comment
    }
    fn surface_type(&self) -> &OpticalSurfaceType {
        &self.surface_type
    }
    fn radius(&self) -> Option<f64> {Some(self.radius)}
    fn thickness(&self) -> Option<f64> { Some(self.thickness)}
    fn position(&self) -> Point3 { self.position }
    fn trace(&self, ray: Ray3) -> Option<Ray3> {trace(ray, self)}
}


#[derive(Default, Debug)]
pub struct SequentialOpticalSystem {
    pub surfaces: Vec<Box<dyn OpticalSurface + 'static>>
}


impl Trace for SequentialOpticalSystem {
    fn trace_ray(&self, mut ray: Ray3) -> Ray3 {
        for surface in self.surfaces.iter() {
            ray = surface.trace(ray).unwrap();
        }
        ray
    }
}


impl SequentialOpticalSystem {
    pub fn add_surface(&mut self, surface: Box<dyn OpticalSurface>) {
        self.surfaces.push(surface)
    }
}


pub fn trace(ray: Ray3, surface: &StandardSurface) -> Option<Ray3> {
    match surface.surface_type() {
        OpticalSurfaceType::Standard => {
            match surface.radius() {
                None => trace_plane(ray, surface),
                Some(_) => trace_sphere(ray, surface)
            }
        }
    };

    Some(ray)
}


pub fn trace_sphere(ray: Ray3, surface: &StandardSurface) -> Option<Ray3> {
    let intersection = ray.intersect_with_sphere(surface.)
    Some(ray)
}


pub fn trace_plane(ray: Ray3, surface: &StandardSurface) -> Option<Ray3> {
    // ray go from origin to Z-plane by default
    // if refraction angle is OK -> trace
    // if refraction is broken - return unit vector to Z with origin from brokent point
    trace_plane_ray(ray, surface.position().z)
}


fn trace_plane_ray(mut ray: Ray3, z: f64) -> Option<Ray3> {
    if ray.propagate_to_z(z) {
        return Some(ray)
    }
    None
}


impl fmt::Display for SequentialOpticalSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f, "N  |   Type   | Comment |  Radius  | Thickness | Material | Semi-diameter\n"
        ).map_err(|err| println!("{:?}", err)).ok();
        for (pos, el) in self.surfaces.iter().enumerate() {
            write!(f, "{}  |", (pos + 1).to_string()).map_err(|err| println!("{:?}", err)).ok();
            write!(f, " {} |", el.surface_type()).map_err(|err| println!("{:?}", err)).ok();
            write!(f, "         |").map_err(|err| println!("{:?}", err)).ok();
            write!(f, " {:.3}   |", el.radius().unwrap_or(0.0)).map_err(|err| println!("{:?}", err)).ok();
            write!(f, " {:.3}   |", el.thickness().unwrap_or(0.0)).map_err(|err| println!("{:?}", err)).ok();
            write!(f, "          |\n").map_err(|err| println!("{:?}", err)).ok();
        }
        Ok(())
    }
}


// #[cfg(test)]
// mod tests {
//     use crate::geometry::vector::Vector3;
//     use super::*;
//
//     #[test]
//     fn test_ray_sphere_intersection() {
//         let c0 = Point3::origin();
//         let c1 = Point3{x: 0.0, y: 3.0, z: 0.0 };
//
//         let rad1: f64 = 1.0;
//         let rad2: f64 = 2.0;
//         let rad3: f64 = 3.0;
//
//         let ray1 = Ray3 {
//             origin: c0,
//             direction: Vector3 {
//                 x: 0.0,
//                 y: 0.0,
//                 z: 1.0,
//             },
//             validity: RayValidity::VALID,
//         };
//         let inter1 = intersect_ray_with_sphere(ray1, c0, rad1);
//         println!("{}", inter1.unwrap());
//     }
// }