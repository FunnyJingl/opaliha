// use num::{Float, NumCast};

pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point2 {
    pub fn origin() -> Point2 {
        Point2 { x: 0., y: 0. }
    }
}

impl Point3 {
    pub fn origin() -> Point3 {
        Point3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl Default for Point3 {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl Default for Point2 {
    fn default() -> Self {
        Self { x: 0., y: 0. }
    }
}
