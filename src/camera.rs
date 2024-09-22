use crate::color::Color;
use crate::interval::Interval;
use crate::point::Point;
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
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
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
        }
    }

    pub fn render(&self, world: &World) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", (self.image_height - j));
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + self.pixel_delta_u * i as f64
                    + self.pixel_delta_v * j as f64;
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let color = self.ray_color(&ray, world);
                println!("{}", color);
            }
        }
        eprintln!("\rDone.                 ");
    }

    fn ray_color(&self, ray: &Ray, world: &World) -> Color {
        if let Some(hit) = world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
            return Color::from(hit.normal + Vec3::new(1.0, 1.0, 1.0) * 0.5);
        }

        let unit_direction = ray.direction.unit_vector();
        let y_direction = unit_direction.y;
        let a = 0.5 * (y_direction + 1.0);
        Color::from(Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a)
    }
}
