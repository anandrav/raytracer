use crate::vec3::Vec3;

pub(crate) type Point = Vec3;

pub(crate) fn random_f64() -> f64 {
    rand::random::<f64>()
}

pub(crate) fn random_f64_in_range(min: f64, max: f64) -> f64 {
    rand::random::<f64>() * (max - min) + min
}
