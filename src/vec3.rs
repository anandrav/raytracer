use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::common::{random_f64, random_f64_in_range};

#[derive(Debug, Copy, Clone)]
pub(crate) struct Vec3 {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl Vec3 {
    pub(crate) fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub(crate) fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub(crate) fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub(crate) fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub(crate) fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub(crate) fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub(crate) fn is_near_zero(&self) -> bool {
        let eps = 1e-8;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }
}

impl Vec3 {
    pub(crate) fn random() -> Self {
        Self {
            x: random_f64(),
            y: random_f64(),
            z: random_f64(),
        }
    }

    pub(crate) fn random_in_range(min: f64, max: f64) -> Self {
        Self {
            x: random_f64_in_range(min, max),
            y: random_f64_in_range(min, max),
            z: random_f64_in_range(min, max),
        }
    }

    pub(crate) fn random_unit() -> Self {
        loop {
            let p = Self::random_in_range(-1.0, 1.0);
            let length_squared = p.length_squared();
            let eps: f64 = 1e-160;
            if eps < length_squared && length_squared <= 1.0 {
                return p;
            }
        }
    }

    pub(crate) fn random_unit_in_hemisphere(normal: Self) -> Self {
        let in_unit_sphere = Self::random_unit();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub(crate) fn reflect(&self, normal: Vec3) -> Vec3 {
        *self - 2.0 * normal * self.dot(normal)
    }

    pub(crate) fn refract(&self, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-*self).dot(normal);
        let cos_theta = cos_theta.min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self::Output {
        Self::Output {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        v * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        self * (1.0 / t)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
