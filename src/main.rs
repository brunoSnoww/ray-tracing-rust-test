mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utils;

use camera::Camera;
use glam::Vec3;
use hittable::hit_world;
use material::{Dielectric, Lambertian, Metal, Scatterable};
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use std::f32::MAX;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

const MAX_DEPTH: i32 = 50;

fn color(r: &Ray, world: &Vec<Sphere>, depth: i32) -> Vec3 {
    if let Some(hit_rec) = hit_world(&world, r, 0.0, MAX) {
        if depth < MAX_DEPTH {
            if let Some((Some(ray), attenuation)) = hit_rec.material.scatter(r, &hit_rec) {
                return attenuation * color(&ray, world, depth + 1);
            }
        }
    }
    let unit_dir = r.direction().normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t)
        * Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
        + t * Vec3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        }
}

fn main() -> std::io::Result<()> {
    let file = Arc::new(Mutex::new(File::create("p.ppm")?));

    let nx = 800;
    let ny = 600;
    let ns = 50;

    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        (nx as f32) / (ny as f32),
    );

    let s1 = Sphere {
        radius: 0.5,
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        material: material::Material::Lambertian(Lambertian {
            albedo: Vec3::new(0.8, 0.3, 0.3),
        }),
    };
    let s2 = Sphere {
        radius: 100.0,
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        material: material::Material::Lambertian(Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.3),
        }),
    };
    let s22 = Sphere {
        radius: 0.5,
        center: Vec3 {
            x: 2.0,
            y: 0.0,
            z: -1.0,
        },
        material: material::Material::Metal(Metal {
            albedo: Vec3::new(0.8, 0.8, 0.3),
            fuzz: 0.5,
        }),
    };
    let s3 = Sphere {
        radius: 0.5,
        center: Vec3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        material: material::Material::Lambertian(Lambertian {
            albedo: Vec3 {
                x: 0.8,
                y: 0.6,
                z: 0.2,
            },
        }),
    };
    let s4 = Sphere {
        radius: -0.5,
        center: Vec3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        material: material::Material::Dielectric(Dielectric { refractionIdx: 1.5 }),
    };

    let world = vec![s1, s2, s3, s4, s22];
    {
        let mut file_lock = file.lock().unwrap();
        file_lock.write(format!("P3{}{} {}{}255{}", '\n', nx, ny, '\n', '\n').as_bytes());
    }
    (0..ny - 1).rev().for_each(|j| {
        (0..nx).for_each(|i| {
            let mut rng = rand::thread_rng();
            let mut col = Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            for _ in 0..ns {
                let dx: f32 = rng.gen_range(0.0..1.0);
                let dy: f32 = rng.gen_range(0.0..1.0);
                let u: f32 = (i as f32 + dx) / (nx as f32);
                let v: f32 = (j as f32 + dy) / (ny as f32);
                let r = camera.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col = col * (1.0 / (ns as f32));
            let ir = (255.99 * col.x.sqrt()) as i64;
            let ig = (255.99 * col.y.sqrt()) as i64;
            let ib = (255.99 * col.z.sqrt()) as i64;
            let s = format!("{} {} {} {}", ir, ig, ib, '\n');
            // Lock around the file writing
            let mut file_lock = file.lock().unwrap();
            file_lock.write_all(s.as_bytes()).unwrap();
        });
    });
    Ok(())
}
