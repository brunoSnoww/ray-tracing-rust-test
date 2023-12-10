use glam::Vec3;
use rand::Rng;

use crate::ray::Ray;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = Vec3::new(0.0, 0.0, 0.0);
    let origin = Vec3::new(1.0, 1.0, 1.0);
    loop {
        let x = rng.gen_range(0.0..1.0);
        let y = rng.gen_range(0.0..1.0);
        let z = rng.gen_range(0.0..1.0);
        p = 2.0 * Vec3::new(x, y, z) - origin;
        if p.dot(p) >= 1.0 {
            break;
        }
    }
    p
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * (*v).dot(*n) * (*n)
}

pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(*n); // cosTHETAONE
    let delta = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if delta > 0.0 {
        Some(ni_over_nt * (*v - (*n) * dt) - ((*n) * delta.sqrt()))
    } else {
        None
    }
}

pub fn schlick(cosine: f32, ridx: f32) -> f32 {
    let r0: f32 = ((1.0 - ridx) / (1.0 + ridx)).powf(2.0);
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
