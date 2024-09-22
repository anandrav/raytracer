use crate::interval::Interval;
use crate::point::Point;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub(crate) struct HitRecord {
    pub(crate) p: Point,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub(crate) fn new(ray: &Ray, p: Point, normal: Vec3, t: f64) -> Self {
        let front_face = ray.direction.dot(normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        Self {
            p,
            normal,
            t,
            front_face: false,
        }
    }
}

pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub(crate) struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub(crate) fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let t = root;
        let p = ray.at(t);
        let normal = (p - self.center) / self.radius;
        Some(HitRecord::new(ray, p, normal, t))
    }
}

pub(crate) struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub(crate) fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub(crate) fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }

    pub(crate) fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_record = None;
        for object in &self.objects {
            if let Some(record) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }
        hit_record
    }
}
