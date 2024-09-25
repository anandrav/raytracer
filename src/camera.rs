use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::color::Color;
use crate::common::random_f64;
use crate::common::Point;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::scene::World;
use crate::vec3::Vec3;
use image::ImageBuffer;
use image::Rgb;
use rayon::prelude::*;
pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,

    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    samples_per_pixel: u16,
    pixel_samples_scale: f64,

    center: Point,
    vertical_fov: f64,
    look_from: Vec3,
    look_at: Vec3,
    up: Vec3,

    u: Vec3,
    v: Vec3,
    w: Vec3,

    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u16,
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        vertical_fov: f64,
        defocus_angle: f64,
        focus_distance: f64,
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height = image_height.clamp(1, u32::MAX);

        let center = look_from;
        // let focal_length = (look_from - look_at).length();
        let w = (look_from - look_at).unit_vector();
        let u = up.cross(w).unit_vector();
        let v = w.cross(u);

        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - (w * focus_distance) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_radius = focus_distance * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            aspect_ratio,
            image_width,
            image_height,

            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,

            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,

            center,
            vertical_fov,
            look_from,
            look_at,
            up,

            u,
            v,
            w,

            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &World) {
        let completed_pixels = Arc::new(AtomicUsize::new(0));

        let total_pixels = self.image_height as usize * self.image_width as usize;

        // Spawn a thread to display progress
        let progress_thread = {
            let completed_scanlines = completed_pixels.clone();
            std::thread::spawn(move || {
                while completed_scanlines.load(Ordering::Relaxed) < total_pixels {
                    let completed = completed_scanlines.load(Ordering::Relaxed);
                    eprint!(
                        "\rProgress: {:.1}%",
                        (completed as f32 / total_pixels as f32) * 100.0
                    );
                    std::thread::sleep(std::time::Duration::from_millis(16));
                }
            })
        };

        // Header for .ppm file
        // println!("P3\n{} {}\n255", self.image_width, self.image_height);

        let scan_lines: Vec<Vec<Color>> = (0..self.image_height)
            .into_par_iter()
            .map(|j| {
                (0..self.image_width)
                    .map(|i| {
                        let mut color = Vec3::new(0.0, 0.0, 0.0);
                        color += (0..self.samples_per_pixel)
                            .map(|_| {
                                let ray = self.get_ray(i as i32, j as i32);
                                ray_color(&ray, world)
                            })
                            .sum();
                        // update progress
                        completed_pixels.fetch_add(1, Ordering::Relaxed);
                        // return
                        Color::from(color * self.pixel_samples_scale)
                    })
                    .collect()
            })
            .collect();

        let mut img = ImageBuffer::new(self.image_width, self.image_height);
        for (j, scan_line) in scan_lines.iter().enumerate() {
            for (i, color) in scan_line.iter().enumerate() {
                img.put_pixel(i as u32, j as u32, Rgb::from([color.r, color.g, color.b]));
            }
        }
        img.save("example.png").unwrap();

        for scan_line in scan_lines {
            for color in scan_line {
                // println!("{}", color);
            }
        }

        progress_thread.join().unwrap();
        eprintln!("\rDone.                 ");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x))
            + (self.pixel_delta_v * (j as f64 + offset.y));

        let origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let direction = pixel_sample - origin;
        Ray { origin, direction }
    }
    fn defocus_disk_sample(&self) -> Vec3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
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
    }

    let unit_direction = ray.direction.unit_vector();
    let y_direction = unit_direction.y;
    let a = 0.5 * (y_direction + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}

fn sample_square() -> Vec3 {
    Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
}
