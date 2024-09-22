mod color;
mod interval;
mod point;
mod ray;
mod shapes;
mod vec3;

use color::Color;
use interval::Interval;
use point::Point;
use ray::Ray;
use shapes::{Hittable, Sphere, World};
use vec3::Vec3;

fn main() {
    let desired_aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / desired_aspect_ratio) as u32;
    let image_height = image_height.clamp(1, u32::MAX);

    // World
    let mut world: World = World::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // Vectors along the viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", (image_height - j));
        for i in 0..image_width {
            let pixel_center = pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(&ray, &world);
            println!("{}", color);
        }
    }
    eprintln!("\rDone.                 ");
}

fn ray_color(ray: &Ray, world: &World) -> Color {
    // let center = Vec3::new(0.0, 0.0, -1.0);
    // let sphere = Sphere::new(center, 0.5);
    // let t = hit_sphere(&center, 0.5, ray);
    if let Some(hit) = world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
        return Color::from(hit.normal + Vec3::new(1.0, 1.0, 1.0) * 0.5);
    }

    let unit_direction = ray.direction.unit_vector();
    let y_direction = unit_direction.y;
    let a = 0.5 * (y_direction + 1.0);
    Color::from(Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a)
}
