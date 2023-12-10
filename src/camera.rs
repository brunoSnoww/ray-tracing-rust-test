use std::f32::consts::PI;

use glam::Vec3;

use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookFrom: Vec3, lookAt: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Self {
        let THETA = vfov * PI / 180.0;
        let half_height = (THETA / 2.0).tan();

        let half_width = aspect * half_height;

        let origin = lookFrom;
        let w = (lookFrom - lookAt).normalize();
        let u = (vup.cross(w)).normalize();
        let v = w.cross(u).normalize();
        let lower_left_corner = origin - (u * half_width) - (v * half_height) - w;

        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;

        return Self {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        };
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let a = self.origin;
        let b = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray { a, b }
    }
}
