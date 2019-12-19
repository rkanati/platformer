
#[derive(Debug, Clone, Copy)]
pub struct Values {
    pub kinetic:   i32,
    pub thermal:   i32,
    pub explosive: i32,
}

fn scale_int(i: i32, k: f64) -> i32 {
    (i as f64 * k) as i32
}

impl Values {
    pub fn resist(self, scales: &Scales) -> Values {
        Values {
            kinetic:   scale_int(self.kinetic,   scales.kinetic),
            thermal:   scale_int(self.thermal,   scales.thermal),
            explosive: scale_int(self.explosive, scales.explosive),
        }
    }

    pub fn sum(&self) -> i32 {
        self.kinetic + self.thermal + self.explosive
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Scales {
    pub kinetic:   f64,
    pub thermal:   f64,
    pub explosive: f64,
}

