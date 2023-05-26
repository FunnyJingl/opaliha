use crate::point::*;

pub struct Vector3 {
    pub p1: Point3,
    pub p2: Point3,
}

pub struct Direction3 {
    pub kx: f64,
    pub ky: f64,
    pub kz: f64,
}

pub struct Unit3 {
    pub p: Point3,
    pub dir: Direction3,
}

impl Unit3 {
    pub fn unit_x() -> Unit3 {
        Unit3 {
            p: Point3::origin(),
            dir: Direction3 {
                kx: 1.,
                ky: 0.,
                kz: 0.,
            },
        }
    }
    pub fn unit_y() -> Unit3 {
        Unit3 {
            p: Point3::origin(),
            dir: Direction3 {
                kx: 0.,
                ky: 1.,
                kz: 0.,
            },
        }
    }
    pub fn unit_z() -> Unit3 {
        Unit3 {
            p: Point3::origin(),
            dir: Direction3 {
                kx: 0.,
                ky: 0.,
                kz: 1.,
            },
        }
    }
}

impl Vector3 {
    pub fn unit_x() -> Vector3 {
        Vector3 {
            p1: Point3::origin(),
            p2: Point3 {
                x: 1.,
                y: 0.,
                z: 0.,
            },
        }
    }
    pub fn unit_y() -> Vector3 {
        Vector3 {
            p1: Point3::origin(),
            p2: Point3 {
                x: 0.,
                y: 1.,
                z: 0.,
            },
        }
    }
    pub fn unit_z() -> Vector3 {
        Vector3 {
            p1: Point3::origin(),
            p2: Point3 {
                x: 0.,
                y: 0.,
                z: 1.,
            },
        }
    }
    pub fn len(&self) -> f64 {
        f64::sqrt(
            (self.p1.x - self.p2.x).powi(2)
                + (self.p1.y - self.p2.y).powi(2)
                + (self.p1.z - self.p2.z).powi(2),
        )
    }

    pub fn direction(&self) -> Unit3 {
        let veclen = self.len();
        Unit3 {
            p: self.p1,
            dir: Direction3 {
                kx: (self.p2.x - self.p1.x) / veclen,
                ky: (self.p2.y - self.p1.y) / veclen,
                kz: (self.p2.z - self.p1.z) / veclen,
            },
        }
    }
}
