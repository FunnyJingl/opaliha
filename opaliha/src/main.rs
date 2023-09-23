mod geometry;
mod optical_surfaces;
mod materials;
// mod optical_system;

use yaml_rust::Yaml::String;
mod optical_system;

fn main() {
    let mut optsys = optical_system::sequential_optical_system::SequentialOpticalSystem::default();
    let surface = optical_system::sequential_optical_system::StandardSurface{
        name: "".to_string(),
        comment: "".to_string(),
        surface_type: optical_system::sequential_optical_system::OpticalSurfaceType::Standard,
        radius: 25.,
        thickness: 7.,
        material: Box::new(materials::material::Air::default())
    };

    let surface = optical_system::sequential_optical_system::StandardSurface{
        name: "".to_string(),
        comment: "".to_string(),
        surface_type: optical_system::sequential_optical_system::OpticalSurfaceType::Standard,
        radius: -25.,
        thickness: 25.,
        material: Box::new(materials::material::Glass{name: "N-BK7".to_string()})
    };

    optsys.add_surface(Box::new(surface));
    println!("{}", optsys);

    println!("Done!");
}
