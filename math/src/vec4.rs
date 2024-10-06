use crate::Mat4;

pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {}

impl std::ops::Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, b: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
            w: self.w + b.w,
        }
    }
}

impl std::ops::Add<f32> for Vec4 {
    type Output = Vec4;

    fn add(self, b: f32) -> Vec4 {
        Vec4 {
            x: self.x + b,
            y: self.y + b,
            z: self.z + b,
            w: self.w + b,
        }
    }
}

impl std::ops::Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, b: Vec4) -> Vec4 {
        Vec4 {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
            w: self.w - b.w,
        }
    }
}

impl std::ops::Sub<f32> for Vec4 {
    type Output = Vec4;

    fn sub(self, b: f32) -> Vec4 {
        Vec4 {
            x: self.x - b,
            y: self.y - b,
            z: self.z - b,
            w: self.w - b,
        }
    }
}

impl std::ops::Mul<Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, b: Vec4) -> Vec4 {
        Vec4 {
            x: self.x * b.x,
            y: self.y * b.y,
            z: self.z * b.z,
            w: self.w * b.w,
        }
    }
}

impl std::ops::Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, b: f32) -> Vec4 {
        Vec4 {
            x: self.x * b,
            y: self.y * b,
            z: self.z * b,
            w: self.w * b,
        }
    }
}

impl std::ops::Mul<Mat4> for Vec4 {
    type Output = Vec4;

    fn mul(self, b: Mat4) -> Vec4 {
        Vec4 {
            x: self.x * b.c0.x + self.y * b.c1.x + self.z * b.c2.x + self.w * b.c3.x,
            y: self.x * b.c0.y + self.y * b.c1.y + self.z * b.c2.y + self.w * b.c3.y,
            z: self.x * b.c0.z + self.y * b.c1.z + self.z * b.c2.z + self.w * b.c3.z,
            w: self.x * b.c0.w + self.y * b.c1.w + self.z * b.c2.w + self.w * b.c3.w,
        }
    }
}

impl std::ops::Div<Vec4> for Vec4 {
    type Output = Vec4;

    fn div(self, b: Vec4) -> Vec4 {
        Vec4 {
            x: self.x / b.x,
            y: self.y / b.y,
            z: self.z / b.z,
            w: self.w / b.w,
        }
    }
}

impl std::ops::Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, b: f32) -> Vec4 {
        Vec4 {
            x: self.x / b,
            y: self.y / b,
            z: self.z / b,
            w: self.w / b,
        }
    }
}

impl std::ops::Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Vec4 {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
