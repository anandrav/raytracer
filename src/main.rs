mod camera;
mod color;
mod common;
mod interval;
mod material;
mod ray;
mod scene;
mod vec3;
use std::{env, sync::Arc};

use camera::Camera;

use common::{random_f64, random_f64_in_range};
use material::{Material, MaterialKind};
use scene::{Sphere, World};
use vec3::Vec3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let samples_per_pixel = if args.len() > 1 {
        args[1].parse::<u16>().unwrap()
    } else {
        10
    };

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        0.6,
        10.0,
    );

    let mut world: World = World::new();

    // World

    // ground
    let material_ground = Arc::new(Material {
        kind: MaterialKind::Lambertian,
        albedo: Vec3::new(0.5, 0.5, 0.5),
    });
    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    ));

    // little spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Vec3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if ((center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9) {
                if (choose_mat < 0.8) {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let material = Arc::new(Material {
                        kind: MaterialKind::Lambertian,
                        albedo,
                    });
                    world.add(Sphere::new(center, 0.2, material));
                } else if (choose_mat < 0.95) {
                    // metal
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = random_f64_in_range(0.0, 0.5);
                    let material = Arc::new(Material {
                        kind: MaterialKind::Metal { fuzz },
                        albedo,
                    });
                    world.add(Sphere::new(center, 0.2, material));
                } else {
                    // glass
                    let dummy_albedo = Vec3::random_in_range(0.5, 1.0);
                    let material = Arc::new(Material {
                        kind: MaterialKind::Dielectric { ref_idx: 1.5 },
                        albedo: dummy_albedo,
                    });
                    world.add(Sphere::new(center, 0.2, material));
                }
            }
        }
    }

    // big spheres
    let material1 = Arc::new(Material {
        kind: material::MaterialKind::Dielectric { ref_idx: 1.5 },
        albedo: Vec3::new(0.8, 0.8, 0.8),
    });
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Material {
        kind: material::MaterialKind::Lambertian,
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Arc::new(Material {
        kind: material::MaterialKind::Metal { fuzz: 0.0 },
        albedo: Vec3::new(0.7, 0.6, 0.5),
    });
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    camera.render(&world);
}
