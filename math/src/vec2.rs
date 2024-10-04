pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Self { x, y }
    }

    pub fn dot(a: Vec2, b: Vec2) -> f32 {
        a.x * b.x + a.y * b.y
    }

    pub fn magnitude(self) -> f32 {
        let a = self.x * self.x;
        let b = self.y * self.y;
        f32::sqrt(a + b)
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, b: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + b.x,
            y: self.y + b.y,
        }
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, b: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - b.x,
            y: self.y - b.y,
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, b: f32) -> Vec2 {
        Vec2 {
            x: self.x * b,
            y: self.y * b,
        }
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, b: f32) -> Vec2 {
        Vec2 {
            x: self.x / b,
            y: self.y / b,
        }
    }
}
