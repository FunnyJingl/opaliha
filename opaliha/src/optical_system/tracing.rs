use crate::{geometry::ray::Ray3, optical_surfaces::optical_surfaces::SequentialOpticalSurface};

pub fn trace(ray: Ray3, surface: SequentialOpticalSurface) -> Ray3 {
    // if surface.radius == 0. {
    //     // this is a plane
    //     // ray.propagate to plane
    //     // do refraction
    // } else {
    //     // lens case
    // }

    ray
}
