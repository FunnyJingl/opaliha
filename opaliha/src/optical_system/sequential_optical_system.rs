use std::fmt::Formatter;
use std::fmt;
use std::os::unix::fs::OpenOptionsExt;
use crate::geometry::ray;
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

pub trait OpticalSurface : std::fmt::Debug {
    fn name(&self) -> &str;
    fn comment(&self) -> &str;
    fn surface_type(&self) -> &OpticalSurfaceType;
}

// #[derive(Default)]
pub struct StandardSurface {
    pub name: String,
    pub comment: String,
    pub surface_type: OpticalSurfaceType,
    pub radius: f64,
    pub thickness: f64,
    pub material: Box<dyn materials::material::Material>
}


pub trait Trace {
    fn trace(&self, ray: Ray3) -> Ray3;
}

impl std::fmt::Debug for StandardSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
}


#[derive(Default, Debug)]
pub struct SequentialOpticalSystem {
    surfaces: Vec<Box<dyn OpticalSurface + 'static>>
}


impl SequentialOpticalSystem {
    pub fn add_surface(&mut self, surface: Box<dyn OpticalSurface>) {
        self.surfaces.push(surface)
    }
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
