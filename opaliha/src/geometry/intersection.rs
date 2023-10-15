use num::Float;
use crate::geometry::point::Point3;
use crate::geometry::ray::Ray3;


pub fn intersect_ray_with_sphere(
    ray: Ray3,
    sphere_origin: Point3,
    sphere_radius: f64
) -> Option<Point3> {
    let vec_ray_sphere = ray.origin - sphere_origin;
    let a: f64 = ray.direction.dot(ray.direction);
    let b: f64 = 2.0 * vec_ray_sphere.dot(ray.direction);
    let c: f64 = vec_ray_sphere.dot(vec_ray_sphere) - sphere_radius * sphere_radius;
    let d: f64 = b * b - 4.0 * a * c;
    if d < 0.0 { return None }
    let t1 = (-b - Float::sqrt(d)) / (2.0 * a);
    let t2 = (-b + Float::sqrt(d)) / (2.0 * a);

    return if t1 < 0.0 {
        if t2 >= 0.0 { Some(ray.origin + ray.direction * t2) } else { None }
    } else {
        if t1 < t2 { Some(ray.origin + ray.direction * t1) } else
        { Some(ray.origin + ray.direction * t2) }
    }
}


#[cfg(test)]
mod tests {
    use crate::geometry::vector::Vector3;
    use crate::geometry::ray::RayValidity;
    use super::*;

    #[test]
    fn test_ray_sphere_intersection() {
        let c0 = Point3::origin();
        let c1 = Point3{x: 0.0, y: 3.0, z: 0.0 };

        let rad1: f64 = 1.0;
        let rad2: f64 = 2.0;
        let rad3: f64 = 3.0;

        let ray1 = Ray3 {
            origin: c0,
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            validity: RayValidity::VALID,
        };
        let ray2 = Ray3 {
            origin: c0,
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            validity: RayValidity::VALID,
        };
        let ray3 = Ray3 {origin: c0, direction: Vector3 {x: 0.0, y: 1.0, z: 0.0},validity: RayValidity::VALID};
        let ray4 = Ray3 {
            origin: c0,
            direction: Vector3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            validity: RayValidity::VALID,
        };
        assert_eq!(intersect_ray_with_sphere(ray1, c0, rad1), Some(Point3{x: 0.0, y: 0.0, z: 1.0}));
        assert_eq!(intersect_ray_with_sphere(ray2, c0, rad1), Some(Point3{x: 0.0, y: 0.0, z: -1.0}));
        assert_eq!(intersect_ray_with_sphere(ray3, c0, rad1), Some(Point3{x: 0.0, y: 1.0, z: 0.0}));
        assert_eq!(intersect_ray_with_sphere(ray4, c0, rad1), Some(Point3{x: 0.0, y: -1.0, z: 0.0}));
        assert_eq!(intersect_ray_with_sphere(ray1, c1, rad1), None);






        println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

        // assert_eq!(inter1.unwrap_or(None), )
        // let ray1 = Ray3 {
        //     origin: Point3 {},
        //     direction: Vector3 {},
        //     validity: RayValidity::VALID,
        // };
        // let ray1 = Ray3 {
        //     origin: Point3 {},
        //     direction: Vector3 {},
        //     validity: RayValidity::VALID,
        // };
        // let ray1 = Ray3 {
        //     origin: Point3 {},
        //     direction: Vector3 {},
        //     validity: RayValidity::VALID,
        // };
        // let ray1 = Ray3 {
        //     origin: Point3 {},
        //     direction: Vector3 {},
        //     validity: RayValidity::VALID,
        // };
        // let ray1 = Ray3 {
        //     origin: Point3 {},
        //     direction: Vector3 {},
        //     validity: RayValidity::VALID,
        // };


        // let c1 = Vector3{x: 1., y: 2., z: 3.};

        // assert_eq!(vector[2], 3.);
    }
}