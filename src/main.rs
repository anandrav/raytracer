mod color;
mod ray;
mod vec3;

use color::Color;
use ray::Ray;
use vec3::Vec3;

type Point = Vec3;

fn main() {
    let desired_aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / desired_aspect_ratio) as u32;
    let image_height = image_height.clamp(1, u32::MAX);

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

            let color = ray_color(&ray);
            println!("{}", color);
        }
    }
    eprintln!("\rDone.                 ");
}

fn ray_color(ray: &Ray) -> Color {
    let center = Vec3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(&center, 0.5, ray);
    if let Some(t) = t {
        let normal = (ray.at(t) - center).unit_vector();
        return Color::from(normal + Vec3::new(1.0, 1.0, 1.0) * 0.5);
    }
    let unit_direction = ray.direction.unit_vector();
    let y_direction = unit_direction.y;
    let a = 0.5 * (y_direction + 1.0);
    Color::from(Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a)
}

fn hit_sphere(center: &Point, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = *center - ray.origin;
    let a = ray.direction.dot(ray.direction);
    let b = -2.0 * ray.direction.dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (2.0 * a))
    }
}
