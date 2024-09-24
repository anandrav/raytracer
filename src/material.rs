use crate::{ray::Ray, scene::HitRecord, vec3::Vec3};

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
                let refracted = unit_direction.refract(hit.normal, ref_idx);

                let scattered = Ray::new(hit.p, refracted);

                Some(Scatter {
                    attenuation,
                    scattered,
                })
            }
        }
    }
}
