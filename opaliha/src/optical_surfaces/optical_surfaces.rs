use crate::materials::glass::Glass;

pub enum SequentialOpticalSurfaceType {
    Standard,
    Image,
}

pub struct SequentialOpticalSurface {
    pub surface_type: SequentialOpticalSurfaceType,
    pub comment: String,
    pub radius: f64,
    pub thickness: f64,
    pub material: Glass,
    pub clear_semi_diameter: f64,
}

impl SequentialOpticalSurface {
    pub fn new(
        surface_type: SequentialOpticalSurfaceType,
        comment: String,
        radius: f64,
        thickness: f64,
        material: Glass,
        clear_semi_diameter: f64,
    ) -> Self {
        Self {
            surface_type,
            comment,
            radius,
            thickness,
            material,
            clear_semi_diameter,
        }
    }
}
