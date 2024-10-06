use crate::Vec4;

pub struct Mat4 {
    pub c0: Vec4,
    pub c1: Vec4,
    pub c2: Vec4,
    pub c3: Vec4,
}

impl Mat4 {}

impl std::ops::Add<f32> for Mat4 {
    type Output = Mat4;

    fn add(self, b: f32) -> Mat4 {
        Mat4 {
            c0: self.c0 + b,
            c1: self.c1 + b,
            c2: self.c2 + b,
            c3: self.c3 + b,
        }
    }
}

impl std::ops::Add<Mat4> for f32 {
    type Output = Mat4;

    fn add(self, b: Mat4) -> Mat4 {
        Mat4 {
            c0: b.c0 + self,
            c1: b.c1 + self,
            c2: b.c2 + self,
            c3: b.c3 + self,
        }
    }
}

impl std::ops::Sub<f32> for Mat4 {
    type Output = Mat4;

    fn sub(self, b: f32) -> Mat4 {
        Mat4 {
            c0: self.c0 - b,
            c1: self.c1 - b,
            c2: self.c2 - b,
            c3: self.c3 - b,
        }
    }
}

impl std::ops::Sub<Mat4> for Mat4 {
    type Output = Mat4;

    fn sub(self, b: Mat4) -> Mat4 {
        Mat4 {
            c0: self.c0 - b.c0,
            c1: self.c1 - b.c1,
            c2: self.c2 - b.c2,
            c3: self.c3 - b.c3,
        }
    }
}

impl std::ops::Mul<f32> for Mat4 {
    type Output = Mat4;

    fn mul(self, b: f32) -> Mat4 {
        Mat4 {
            c0: self.c0 * b,
            c1: self.c1 * b,
            c2: self.c2 * b,
            c3: self.c3 * b,
        }
    }
}

impl std::ops::Mul<Mat4> for f32 {
    type Output = Mat4;

    fn mul(self, b: Mat4) -> Mat4 {
        Mat4 {
            c0: b.c0 * self,
            c1: b.c1 * self,
            c2: b.c2 * self,
            c3: b.c3 * self,
        }
    }
}

impl std::ops::Neg for Mat4 {
    type Output = Mat4;

    fn neg(self) -> Mat4 {
        Mat4 {
            c0: -self.c0,
            c1: -self.c1,
            c2: -self.c2,
            c3: -self.c3,
        }
    }
}
