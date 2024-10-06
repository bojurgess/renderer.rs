use crate::{Vec2, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat3 {
    pub c0: Vec3,
    pub c1: Vec3,
    pub c2: Vec3,
}

impl Mat3 {
    pub const ZERO: Mat3 = Mat3 {
        c0: Vec3::ZERO,
        c1: Vec3::ZERO,
        c2: Vec3::ZERO,
    };

    pub const IDENTITY: Mat3 = Mat3 {
        c0: Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        c1: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        c2: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };

    pub fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
    ) -> Mat3 {
        Mat3 {
            c0: Vec3::new(m00, m01, m02),
            c1: Vec3::new(m10, m11, m12),
            c2: Vec3::new(m20, m21, m22),
        }
    }

    pub fn from_cols(c0: Vec3, c1: Vec3, c2: Vec3) -> Mat3 {
        Mat3 { c0, c1, c2 }
    }

    pub fn from_array(arr: [f32; 9]) -> Mat3 {
        Mat3 {
            c0: Vec3::new(arr[0], arr[1], arr[2]),
            c1: Vec3::new(arr[3], arr[4], arr[5]),
            c2: Vec3::new(arr[6], arr[7], arr[8]),
        }
    }

    // + - +
    // - + -
    // + - +
    pub fn determinant(self) -> f32 {
        let a = self.c0.x * ((self.c1.y * self.c2.z) - (self.c2.y * self.c1.z));
        let b = self.c1.x * ((self.c0.y * self.c2.z) - (self.c2.y * self.c0.z));
        let c = self.c2.x * ((self.c0.y * self.c1.z) - (self.c1.y * self.c0.z));

        a - b + c
    }

    pub fn transpose(self) -> Mat3 {
        Mat3 {
            c0: Vec3::new(self.c0.x, self.c1.x, self.c2.x),
            c1: Vec3::new(self.c0.y, self.c1.y, self.c2.y),
            c2: Vec3::new(self.c0.z, self.c1.z, self.c2.z),
        }
    }

    pub fn inverse(self) -> Option<Mat3> {
        let det = self.determinant();
        let singular = det == 0.0;

        if singular {
            return None;
        }

        let inv_det = 1.0 / det;

        let minors = Mat3 {
            c0: Vec3::new(
                (self.c1.y * self.c2.z) - (self.c2.y * self.c1.z),
                (self.c1.x * self.c2.z) - (self.c2.x * self.c1.z),
                (self.c1.x * self.c2.y) - (self.c2.x * self.c1.y),
            ),
            c1: Vec3::new(
                (self.c0.y * self.c2.z) - (self.c2.y * self.c0.z),
                (self.c0.x * self.c2.z) - (self.c2.x * self.c0.z),
                (self.c0.x * self.c2.y) - (self.c2.x * self.c0.y),
            ),
            c2: Vec3::new(
                (self.c0.y * self.c1.z) - (self.c1.y * self.c0.z),
                (self.c0.x * self.c1.z) - (self.c1.x * self.c0.z),
                (self.c0.x * self.c1.y) - (self.c1.x * self.c0.y),
            ),
        };

        let adjugate = Mat3 {
            c0: Vec3::new(minors.c0.x, -minors.c1.x, minors.c2.x),
            c1: Vec3::new(-minors.c0.y, minors.c1.y, -minors.c2.y),
            c2: Vec3::new(minors.c0.z, -minors.c1.z, minors.c2.z),
        };

        Some(adjugate.transpose() * inv_det)
    }

    pub fn rotate(self, theta: f32) -> Mat3 {
        let c = f32::cos(theta);
        let s = f32::sin(theta);

        Mat3 {
            c0: Vec3::new(c, -s, 0.0),
            c1: Vec3::new(s, c, 0.0),
            c2: Vec3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn scale(scale: Vec2) -> Mat3 {
        Mat3 {
            c0: Vec3::new(scale.x, 0.0, 0.0),
            c1: Vec3::new(0.0, scale.y, 0.0),
            c2: Vec3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn translate(translation: Vec2) -> Mat3 {
        Mat3 {
            c0: Vec3::new(1.0, 0.0, 0.0),
            c1: Vec3::new(0.0, 1.0, 0.0),
            c2: Vec3::new(translation.x, translation.y, 1.0),
        }
    }
}

impl std::ops::Add<f32> for Mat3 {
    type Output = Mat3;

    fn add(self, b: f32) -> Mat3 {
        Mat3 {
            c0: self.c0 + b,
            c1: self.c1 + b,
            c2: self.c2 + b,
        }
    }
}

impl std::ops::Add<Mat3> for f32 {
    type Output = Mat3;

    fn add(self, b: Mat3) -> Mat3 {
        Mat3 {
            c0: b.c0 + self,
            c1: b.c1 + self,
            c2: b.c2 + self,
        }
    }
}

impl std::ops::Sub<f32> for Mat3 {
    type Output = Mat3;

    fn sub(self, b: f32) -> Mat3 {
        Mat3 {
            c0: self.c0 - b,
            c1: self.c1 - b,
            c2: self.c2 - b,
        }
    }
}

impl std::ops::Sub<Mat3> for Mat3 {
    type Output = Mat3;

    fn sub(self, b: Mat3) -> Mat3 {
        Mat3 {
            c0: self.c0 - b.c0,
            c1: self.c1 - b.c1,
            c2: self.c2 - b.c2,
        }
    }
}

impl std::ops::Mul<f32> for Mat3 {
    type Output = Mat3;

    fn mul(self, b: f32) -> Mat3 {
        Mat3 {
            c0: self.c0 * b,
            c1: self.c1 * b,
            c2: self.c2 * b,
        }
    }
}

impl std::ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, b: Mat3) -> Mat3 {
        let c0 = Vec3::new(
            Vec3::dot(self.c0, Vec3::new(b.c0.x, b.c1.x, b.c2.x)),
            Vec3::dot(self.c0, Vec3::new(b.c0.y, b.c1.y, b.c2.y)),
            Vec3::dot(self.c0, Vec3::new(b.c0.z, b.c1.z, b.c2.z)),
        );

        let c1 = Vec3::new(
            Vec3::dot(self.c1, Vec3::new(b.c0.x, b.c1.x, b.c2.x)),
            Vec3::dot(self.c1, Vec3::new(b.c0.y, b.c1.y, b.c2.y)),
            Vec3::dot(self.c1, Vec3::new(b.c0.z, b.c1.z, b.c2.z)),
        );

        let c2 = Vec3::new(
            Vec3::dot(self.c2, Vec3::new(b.c0.x, b.c1.x, b.c2.x)),
            Vec3::dot(self.c2, Vec3::new(b.c0.y, b.c1.y, b.c2.y)),
            Vec3::dot(self.c2, Vec3::new(b.c0.z, b.c1.z, b.c2.z)),
        );

        Mat3 { c0, c1, c2 }
    }
}

impl std::ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, b: Vec3) -> Vec3 {
        Vec3::new(
            Vec3::dot(self.c0, b),
            Vec3::dot(self.c1, b),
            Vec3::dot(self.c2, b),
        )
    }
}

impl std::ops::Div<f32> for Mat3 {
    type Output = Mat3;

    fn div(self, b: f32) -> Mat3 {
        Mat3 {
            c0: self.c0 / b,
            c1: self.c1 / b,
            c2: self.c2 / b,
        }
    }
}

impl std::ops::Div<Mat3> for Mat3 {
    type Output = Mat3;

    fn div(self, b: Mat3) -> Mat3 {
        self * b.inverse().unwrap()
    }
}

impl std::ops::Add<Mat3> for Mat3 {
    type Output = Mat3;

    fn add(self, b: Mat3) -> Mat3 {
        Mat3 {
            c0: self.c0 + b.c0,
            c1: self.c1 + b.c1,
            c2: self.c2 + b.c2,
        }
    }
}

impl std::ops::Neg for Mat3 {
    type Output = Mat3;

    fn neg(self) -> Mat3 {
        Mat3 {
            c0: -self.c0,
            c1: -self.c1,
            c2: -self.c2,
        }
    }
}
