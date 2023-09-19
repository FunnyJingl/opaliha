use crate::geometry::ray;
// use crate::optical_surfaces::optical_surfaces::{self, SequentialOpticalSurface};
// use crate::optical_system::parameters;
//
// pub struct SequentialOpticalSystem {
//     surfaces: Vec<Box<optical_surfaces::SequentialOpticalSurface>>,
//     pub parameters: parameters::SequentialParameters,
// }
//
// impl SequentialOpticalSystem {
//     pub fn add_surface(&mut self, surface: SequentialOpticalSurface) {
//         self.surfaces.push(Box::new(surface));
//     }
//
//     pub fn remove_surface(&mut self, idx: Option<i32>) {
//         let idx = idx.unwrap_or(self.get_size() - 1);
//         if idx > 0 {
//             self.surfaces.remove(idx as usize);
//         }
//     }
//
//     fn get_size(&self) -> i32 {
//         self.surfaces.len() as i32
//     }
// }


use clap::builder::Str;
use crate::geometry::ray::Ray3;

pub enum OpticalSurfaceType {
    Biconic,
    BiconicZernike,
    ChebyshevPolynomial,
    EvenAsphere,
    ExtendedAsphere,
    ExtendedOddAsphere,
    ExtendedPolynomial,
    GridSag,
    Irregular,
    OddAsphere,
    OddCosine,
    OffAxisConicFreeform,
    Periodic,
    Polynomial,
    QTypeAsphere,
    QTypeFreeform,
    Standard,
    Superconic,
    Tilted,
    Toroidal,
    ZernikeFringeSag,
    ZernikeStandardSag,
    ZernikeAnnularStandardSag
}


pub trait OpticalSurface {
    fn name(&self) -> &str;
    fn comment(&self) -> &str;
    fn surface_type(&self) -> &OpticalSurfaceType;
}


pub struct StandardSurface {
    pub name: String,
    pub comment: String,
    pub surface_type: OpticalSurfaceType
}


pub trait Trace {
    fn trace(&self, ray: Ray3) -> Ray3;
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

pub struct OpticalSystem {
    surfaces: Vec<Box<dyn OpticalSurface + 'static>>
}