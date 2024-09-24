use crate::{common::random_f64, ray::Ray, scene::HitRecord, vec3::Vec3};

pub struct Material {
    pub kind: MaterialKind,
    pub albedo: Vec3,
}

pub enum MaterialKind {
    Lambertian,
    Metal { fuzz: f64 },
    Dielectric { ref_idx: f64 },
}

pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<Scatter> {
        match self.kind {
            MaterialKind::Lambertian => {
                let target = hit.p + hit.normal + Vec3::random_unit();
                let mut direction = target - hit.p;
                if direction.is_near_zero() {
                    direction = hit.normal;
                }
                let scattered = Ray::new(hit.p, direction);

                Some(Scatter {
                    attenuation: self.albedo,
                    scattered,
                })
            }
            MaterialKind::Metal { fuzz } => {
                let reflected = ray_in.direction.reflect(hit.normal);
                let reflected = reflected.unit_vector() + Vec3::random_unit() * fuzz;
                let scattered = Ray::new(hit.p, reflected);

                Some(Scatter {
                    attenuation: self.albedo,
                    scattered,
                })
            }
            MaterialKind::Dielectric { ref_idx } => {
                let attenuation = Vec3::new(1.0, 1.0, 1.0);
                let ref_idx = if hit.front_face {
                    1.0 / ref_idx
                } else {
                    ref_idx
                };

                let unit_direction = ray_in.direction.unit_vector();
                let cos_theta = (-unit_direction).dot(hit.normal);
                let cos_theta = cos_theta.min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = ref_idx * sin_theta > 1.0;
                let direction = if cannot_refract || reflectance(cos_theta, ref_idx) > random_f64()
                {
                    unit_direction.reflect(hit.normal)
                } else {
                    unit_direction.refract(hit.normal, ref_idx)
                };

                let scattered = Ray::new(hit.p, direction);

                Some(Scatter {
                    attenuation,
                    scattered,
                })
            }
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
