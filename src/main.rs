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

use material::Material;
use scene::{Sphere, World};
use vec3::Vec3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let samples_per_pixel = if args.len() > 1 {
        args[1].parse::<u16>().unwrap()
    } else {
        100
    };

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        10.0,
        3.4,
    );

    // World
    let material_ground = Arc::new(Material {
        kind: material::MaterialKind::Lambertian,
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });
    let material_center = Arc::new(Material {
        kind: material::MaterialKind::Lambertian,
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });
    let material_left = Arc::new(Material {
        kind: material::MaterialKind::Dielectric { ref_idx: 1.5 },
        albedo: Vec3::new(0.8, 0.8, 0.8),
    });
    let material_bubble = Arc::new(Material {
        kind: material::MaterialKind::Dielectric { ref_idx: 1.0 / 1.5 },
        albedo: Vec3::new(0.8, 0.8, 0.8),
    });
    let material_right = Arc::new(Material {
        kind: material::MaterialKind::Metal { fuzz: 1.0 },
        albedo: Vec3::new(0.8, 0.6, 0.2),
    });

    let mut world: World = World::new();
    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    ));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

    camera.render(&world);
}
