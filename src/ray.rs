use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Ray {
    pub(crate) origin: Vec3,
    pub(crate) direction: Vec3,
}

impl Ray {
    pub(crate) fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub(crate) fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
