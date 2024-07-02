// mod geometry;
// mod optical_surfaces;
// mod materials;
// use crate::geometry::point::Point3;
// use crate::geometry::ray::{Ray3, RayValidity};
// use crate::geometry::vector::Vector3;
// use crate::optical_system::sequential_optical_system::Trace;
//
// mod optical_system;
//
// fn main() {
//     let mut optsys = optical_system::sequential_optical_system::SequentialOpticalSystem::default();
//
//     let surface_0 = optical_system::sequential_optical_system::StandardSurface{
//         name: "".to_string(),
//         comment: "".to_string(),
//         surface_type: optical_system::sequential_optical_system::OpticalSurfaceType::Standard,
//         radius: 0.0,
//         thickness: 7.,
//         material: Box::new(materials::material::Air::default()),
//         position: Point3{x: 0., y: 0., z: 0.}
//     };
//
//     let surface_1 = optical_system::sequential_optical_system::StandardSurface{
//         name: "".to_string(),
//         comment: "".to_string(),
//         surface_type: optical_system::sequential_optical_system::OpticalSurfaceType::Standard,
//         radius: 25.,
//         thickness: 7.,
//         material: Box::new(materials::material::Air::default()),
//         position: Point3{x: 0., y: 0., z: 0.}
//     };
//
//     let surface_2 = optical_system::sequential_optical_system::StandardSurface{
//         name: "".to_string(),
//         comment: "".to_string(),
//         surface_type: optical_system::sequential_optical_system::OpticalSurfaceType::Standard,
//         radius: -25.,
//         thickness: 25.,
//         material: Box::new(materials::material::Glass{name: "N-BK7".to_string()}),
//         position: Point3{x: 0., y: 0., z: 7.}
//     };
//
//     optsys.add_surface(Box::new(surface_1));
//     optsys.add_surface(Box::new(surface_2));
//
//     let mut r = Ray3{
//         origin: Point3{x: 0., y: 0., z: -50.},
//         direction: Vector3{x: 0., y: 5., z: 50.},
//         validity: RayValidity::VALID
//     };
//
//     r = optsys.trace_ray(r);
//
//     println!("{}", optsys);
//     println!("{}", r);
//
//     println!("Done!");
// }


use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
