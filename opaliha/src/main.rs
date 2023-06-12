mod geometry;
// use crate::geometry;
use geometry::point;

use crate::materials::glass::Glass;
mod optical_surfaces;
// use optical_surfaces::optical_surfaces;
mod materials;
mod optical_system;

fn main() {
    let mut point = point::Point3 {
        x: 0.,
        y: 0.,
        z: 1.,
    };

    let mut v = geometry::vector::Vector3::unit_y();
    v.p1.x = 0.1;

    // determine a default system with corresponding parameters
    // let mut optical_system_default = optical_system::OpticalSystem;

    let mut surface1 = optical_surfaces::optical_surfaces::SequentialOpticalSurface {
        surface_type: optical_surfaces::optical_surfaces::SequentialOpticalSurfaceType::Standard,
        radius: 25.,
        thickness: 5.,
        material: Glass::new(String::from("K8")),
        clear_semi_diameter: 10.,
        comment: String::from(""),
    };

    // optical_system = OpticalSystem.load_from_yaml();
    // OR
    // optical_system.add(Surface1())
    // optical_system.add(Surface2())
    // optical_system.add(Surface3())
    // optical_system.set_wv(wv1, wv2); ??
    // os.print()
    // trace rays into image plain
    // ::optical_system.trace();
    // visualize_tracing;
    // calculate aberrations

    println!("Done!");
}
