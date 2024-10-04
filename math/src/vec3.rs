#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
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
