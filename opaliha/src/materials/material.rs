use std::fmt;


pub trait Material {
    fn name(&self) -> &str;
    fn refraction_index_at(&self, wavelength: f64) -> f64;
}

pub struct Air {
    refraction_index: f64,
    name: String,
}


impl Default for Air {
    fn default() -> Self {
        Air{refraction_index: 1.0, name: "air".to_string() }
    }
}

pub struct Glass {
    pub name: String,
}

impl Material for Glass {
    fn name(&self) -> &str {
        &self.name
    }

    fn refraction_index_at(&self, wavelength: f64) -> f64 {
        // apply lerp here for already readed data,
        // if wavelength outside range -> raise error
        1.
    }
}


impl Material for Air {
    fn name(&self) -> &str {
        &self.name
    }

    fn refraction_index_at(&self, wavelength: f64) -> f64 {
        1.
    }

}

