use crate::Mat3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Self { x, y, z }
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    // i = (a.y * b.z) - (b.y * a.z)
    // j = (a.x * b.z) - (b.x * a.z)
    // k = (a.x * b.y) - (b.x * a.y)
    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        let x = (a.y * b.z) - (b.y * a.z);
        let y = (a.x * b.z) - (b.x * a.z);
        let z = (a.x * b.y) - (b.x * a.y);

        Vec3 { x, y, z }
    }

    pub fn magnitude(self) -> f32 {
        let a = self.x * self.x;
        let b = self.y * self.y;
        let c = self.z * self.z;
        f32::sqrt(a + b + c)
    }

    pub fn normalise(self) -> Vec3 {
        let m = self.magnitude();
        Vec3 {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
        }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        }
    }
}

impl std::ops::Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, b: f32) -> Vec3 {
        Vec3 {
            x: self.x + b,
            y: self.y + b,
            z: self.z + b,
        }
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }
}

impl std::ops::Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, b: f32) -> Vec3 {
        Vec3 {
            x: self.x - b,
            y: self.y - b,
            z: self.z - b,
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, b: f32) -> Vec3 {
        Vec3 {
            x: self.x * b,
            y: self.y * b,
            z: self.z * b,
        }
    }
}

impl std::ops::Mul<Mat3> for Vec3 {
    type Output = Vec3;

    fn mul(self, b: Mat3) -> Vec3 {
        let x = Vec3::dot(self, Vec3::new(b.c0.x, b.c1.x, b.c2.x));
        let y = Vec3::dot(self, Vec3::new(b.c0.y, b.c1.y, b.c2.y));
        let z = Vec3::dot(self, Vec3::new(b.c0.z, b.c1.z, b.c2.z));

        Vec3 { x, y, z }
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * b.x,
            y: self.y * b.y,
            z: self.z * b.z,
        }
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, b: f32) -> Vec3 {
        Vec3 {
            x: self.x / b,
            y: self.y / b,
            z: self.z / b,
        }
    }
}

impl std::ops::Div<Mat3> for Vec3 {
    type Output = Vec3;

    fn div(self, b: Mat3) -> Vec3 {
        let x = Vec3::dot(self, Vec3::new(b.c0.x, b.c0.y, b.c0.z));
        let y = Vec3::dot(self, Vec3::new(b.c1.x, b.c1.y, b.c1.z));
        let z = Vec3::dot(self, Vec3::new(b.c2.x, b.c2.y, b.c2.z));

        Vec3 { x, y, z }
    }
}

impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / b.x,
            y: self.y / b.y,
            z: self.z / b.z,
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
