use crate::optical_surfaces::optical_surfaces::{self, SequentialOpticalSurface};
use crate::optical_system::parameters;

pub struct SequentialOpticalSystem {
    surfaces: Vec<Box<optical_surfaces::SequentialOpticalSurface>>,
    pub parameters: parameters::SequentialParameters,
}

impl SequentialOpticalSystem {
    pub fn add_surface(&mut self, surface: SequentialOpticalSurface) {
        self.surfaces.push(Box::new(surface));
    }

    pub fn remove_surface(&mut self, idx: Option<i32>) {
        let idx = idx.unwrap_or(self.get_size() - 1);
        if idx > 0 {
            self.surfaces.remove(idx as usize);
        }
    }

    fn get_size(&self) -> i32 {
        self.surfaces.len() as i32
    }
}
