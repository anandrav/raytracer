mod camera;
mod color;
mod interval;
mod common;
mod ray;
mod scene;
mod vec3;

use camera::Camera;
use color::Color;
use interval::Interval;
use common::Point;
use ray::Ray;
use scene::{Hittable, Sphere, World};
use vec3::Vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let camera = Camera::new(aspect_ratio, image_width);

    // World
    let mut world: World = World::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    camera.render(&world);
}
