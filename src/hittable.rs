use glam::Vec3;

use crate::{material::Material, ray::Ray, sphere::Sphere};

#[derive(Clone)]
pub struct hit_record<'material> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'material Material,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<hit_record>;
}

pub fn hit_world<'material>(
    world: &'material Vec<Sphere>,
    r: &Ray,
    t_min: f32,
    t_max: f32,
) -> Option<hit_record<'material>> {
    let mut closest_so_far = t_max;
    let mut hit_record = None;
    for sphere in world {
        if let Some(hit) = sphere.hit(r, t_min, closest_so_far) {
            closest_so_far = hit.t;
            hit_record = Some(hit);
        }
    }
    hit_record
}
