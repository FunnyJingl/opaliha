use std::fmt::Formatter;
use std::fmt;
use crate::geometry::point::Point3;
use crate::materials;
use crate::geometry::ray::Ray3;

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
    fn position(&self) -> Point3;
    fn trace(&self, ray: Ray3) -> Ray3;
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


pub trait Trace {
    fn trace(&self, ray: Ray3) -> Ray3;
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
    fn position(&self) -> Point3 { self.position }
    fn trace(&self, ray: Ray3) {self.trace(ray)}
}


#[derive(Default, Debug)]
pub struct SequentialOpticalSystem {
    pub surfaces: Vec<Box<dyn OpticalSurface + 'static>>
}


impl Trace for SequentialOpticalSystem {
    fn trace(&self, mut ray: Ray3) -> Ray3 {
        for surface in self.surfaces.iter() {
            ray = surface.trace(ray);
        }
        ray
    }
}


impl SequentialOpticalSystem {
    pub fn add_surface(&mut self, surface: Box<dyn OpticalSurface>) {
        self.surfaces.push(surface)
    }
}


pub fn trace(ray: Ray3, surface: Box<dyn OpticalSurface>) {
    if surface.surface_type() == OpticalSurfaceType::Standard {
        match surface.radius() {
            None => trace_plane(ray, surface),
            Some(r) => panic!()
        }
    }
}


pub fn trace_plane(ray: Ray3, surface: Box<dyn OpticalSurface>) -> Option<Ray3> {
    // ray go from origin to Z-plane by default
    // if refraction angle is OK -> trace
    // if refraction is broken - return unit vector to Z with origin from brokent point
    trace_plane_ray(ray, surface.position().z)
}


fn trace_plane_ray(mut ray: Ray3, z: f64) -> Option<Ray3> {
    if ray.propagate_to_z(z) {
        ray
    }
    None
}


impl fmt::Display for SequentialOpticalSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (pos, el) in self.surfaces.iter().enumerate() {
            write!(
                f, "N  |   Type   | Comment | Radius | Thickness | Material | Semi-diameter\n"
            ).map_err(|err| println!("{:?}", err)).ok();
            write!(f, "{}  |", (pos + 1).to_string()).map_err(|err| println!("{:?}", err)).ok();
            write!(f, " {} |", el.surface_type()).map_err(|err| println!("{:?}", err)).ok();
            write!(f, "         | ").map_err(|err| println!("{:?}", err)).ok();
        }
        Ok(())
    }
}
