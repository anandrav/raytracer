use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        let r = (255.999 * v.x) as u8;
        let g = (255.999 * v.y) as u8;
        let b = (255.999 * v.z) as u8;
        Self { r, g, b }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}
