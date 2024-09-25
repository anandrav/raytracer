use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl Color {
    pub(crate) fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        let to_gamma = |f: f64| {
            if f > 0.0 {
                f.sqrt()
            } else {
                0.0
            }
        };
        let to_int = |f: f64| (255.999 * f.clamp(0.0, 0.999)) as u8;
        let convert = |f: f64| to_int(to_gamma(f));

        let r = convert(v.x);
        let g = convert(v.y);
        let b = convert(v.z);
        Self { r, g, b }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}
