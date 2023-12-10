use crate::hittable::hit_record;
use crate::hittable::Hittable;
use crate::material::Material;
use glam::Vec3;
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, r: f32, material: Material) -> Self {
        Self {
            center,
            radius: r,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<hit_record> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(oc) - (self.radius * self.radius);
        let delta = b * b - 4.0 * a * c;

        if delta > 0.0 {
            let (r1, r2) = (
                (-b - delta.sqrt()) / (2.0 * a),
                (-b + delta.sqrt()) / (2.0 * a),
            );
            for root in [r1, r2].iter() {
                if *root < t_max && *root > t_min {
                    let p = ray.point_at_parameter(*root);
                    let normal = (p - self.center) * (1.0 / self.radius);
                    return Some(hit_record {
                        p,
                        normal,
                        t: *root,
                        material: &self.material,
                    });
                }
            }
        }
        None
    }
}
