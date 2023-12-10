use std::ops::Mul;
pub struct Ray {
    pub a: glam::Vec3,
    pub b: glam::Vec3,
}

impl Ray {
    pub fn new(a: glam::Vec3, b: glam::Vec3) -> Self {
        Self { a, b }
    }
    pub fn origin(&self) -> glam::Vec3 {
        self.a
    }
    pub fn direction(&self) -> glam::Vec3 {
        self.b
    }
    pub fn point_at_parameter(&self, t: f32) -> glam::Vec3 {
        self.a + self.b.mul(t)
    }
}
