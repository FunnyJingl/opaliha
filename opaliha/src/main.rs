// struct Glass;
// struct OpticalElement;
// struct Lens;
// struct LensBiConvex;
// struct LensBiConcave;
// struct OpticalSystemSequential;
// struct OpticalSystem;
// struct Detector;
// struct Scene;
// struct Config;
mod optical_elements;
mod optical_surfaces;

use std::collections::HashMap;

enum TypeOpticalSystem {
    Sequential,
}

enum SurfaceType {
    Plane,
    Radius,
}

pub struct Glass {
    name: String,
}

pub struct SequentialOpticalSurface {
    surface_type: SurfaceType,
    comment: String,
    thickness: f32,
    semi_diameter: f32,
    glass: Glass,
}

pub struct SequentialOpticalSystem {}

trait OpticalSystem {
    fn type_optical_system(&self) -> TypeOpticalSystem;
}

// impl OpticalSystem for SequentialOpticalSystem {
//     fn type_optical_system(&self) {
//         TypeOpticalSystem::Sequential
//     }
// }

impl SequentialOpticalSurface {}

fn main() {
    println!("Hello, world!");

    let sos = SequentialOpticalSystem {};

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.entry(String::from("Yellow")).or_insert(100);

    let text = String::from("world world aa world");
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
