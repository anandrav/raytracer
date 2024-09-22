pub(crate) struct Interval {
    pub(crate) min: f64,
    pub(crate) max: f64,
}

impl Interval {
    const EMPTY: Self = Self {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    const UNIVERSE: Self = Self {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    pub(crate) fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub(crate) fn size(&self) -> f64 {
        self.max - self.min
    }

    pub(crate) fn contains(&self, t: f64) -> bool {
        self.min <= t && t <= self.max
    }

    pub(crate) fn surrounds(&self, t: f64) -> bool {
        self.min < t && t < self.max
    }
}
