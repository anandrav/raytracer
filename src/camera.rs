use crate::color::Color;
use crate::common::random_f64;
use crate::common::Point;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::scene::World;
use crate::vec3::Vec3;
pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u16,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u16) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height = image_height.clamp(1, u32::MAX);

        let center = Point::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
        }
    }

    pub fn render(&self, world: &World) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", (self.image_height - j));
            for i in 0..self.image_width {
                let mut color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i as i32, j as i32);
                    color += ray_color(&ray, world);
                }
                println!("{}", Color::from(color * self.pixel_samples_scale));
            }
        }
        eprintln!("\rDone.                 ");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x))
            + (self.pixel_delta_v * (j as f64 + offset.y));

        let origin = self.center;
        let direction = pixel_sample - self.center;
        Ray { origin, direction }
    }
}

fn ray_color(ray: &Ray, world: &World) -> Vec3 {
    ray_color_(ray, world, 0)
}

fn ray_color_(ray: &Ray, world: &World, depth: i32) -> Vec3 {
    const MAX_DEPTH: i32 = 50;
    if depth >= MAX_DEPTH {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
        match hit.material.scatter(ray, &hit) {
            Some(scatter) => {
                return scatter.attenuation * ray_color_(&scatter.scattered, world, depth + 1);
            }
            None => return Vec3::new(0.0, 0.0, 0.0),
        }
        // let direction = hit.normal + Vec3::random_unit();
        // return ray_color_(&Ray::new(hit.p, direction), world, depth + 1) * 0.5;
    }

    let unit_direction = ray.direction.unit_vector();
    let y_direction = unit_direction.y;
    let a = 0.5 * (y_direction + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}

fn sample_square() -> Vec3 {
    Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
}
