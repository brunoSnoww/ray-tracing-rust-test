use glam::Vec3;
use rand::Rng;

use crate::{
    hittable::{self, hit_record},
    ray::Ray,
    utils::{random_in_unit_sphere, reflect, refract, schlick},
};

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &hit_record) -> Option<(Option<Ray>, Vec3)>;
}
#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vec3,
}

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &hit_record) -> Option<(Option<Ray>, Vec3)> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metal(m) => m.scatter(ray, hit_record),
            Material::Dielectric(d) => d.scatter(ray, hit_record),
        }
    }
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &hit_record) -> Option<(Option<Ray>, Vec3)> {
        let mut scatter_direction = hit_record.normal + random_in_unit_sphere();
        let target = hit_record.p + scatter_direction;
        let scattered = Ray::new(hit_record.p, target - hit_record.p);
        let attenuation = self.albedo;
        Some((Some(scattered), attenuation))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, f: f32) -> Self {
        let fuzz = f.min(1.0);
        Self { albedo, fuzz }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, rec: &hit_record) -> Option<(Option<Ray>, Vec3)> {
        let reflected = reflect(&ray.direction(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        let attenuation = self.albedo;
        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((Some(scattered), attenuation))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    pub refractionIdx: f32,
}
impl Scatterable for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &hit_record) -> Option<(Option<Ray>, Vec3)> {
        let mut rng = rand::thread_rng();
        let attenuation = Vec3::new(1.0 as f32, 1.0 as f32, 1.0 as f32);
        let refraction_ratio = if ray.direction().dot(hit_record.normal) < 0.0 {
            1.0 / self.refractionIdx
        } else {
            self.refractionIdx
        };
        let unit_direction = ray.direction().normalize();
        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0
            || schlick(cos_theta, refraction_ratio) > rng.gen::<f32>();

        if cannot_refract {
            let reflected = reflect(&unit_direction, &hit_record.normal);
            let scattered = Ray::new(hit_record.p, reflected);
            Some((Some(scattered), attenuation))
        } else {
            let direction = refract(&unit_direction, &hit_record.normal, refraction_ratio);
            let scattered = Ray::new(hit_record.p, direction.unwrap());
            Some((Some(scattered), attenuation))
        }
    }
}
