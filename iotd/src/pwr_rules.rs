use core::ops::Range;

pub struct PowerLimit {
    band: Range<u32>,
    max_pwr: f32,
    min_pwr: f32
}

impl PowerLimit {
    pub fn limit(&self, p: f32, f: u32) -> Option<f32> {
        if !self.band.contains(&f) { return None };
        if p <= self.min_pwr { return Some(self.min_pwr) }
        if p >= self.max_pwr { return Some(self.max_pwr) }
        return Some(p)
    }
}

const DEFAULT_PWR_LIMIT: [PowerLimit; 1] = [PowerLimit{band: 169000000..170000000, min_pwr: -5.0f32, max_pwr: -5.0f32}];
const IT_PWR_LIMIT: [PowerLimit; 1] = [PowerLimit{band: 169000000..170000000, min_pwr: -5.0f32, max_pwr: 20.0f32}];

pub const COUNTRIES_LUT: [(&str, &[PowerLimit]); 2] = [
    ("IT", &IT_PWR_LIMIT),

    ("Default", &DEFAULT_PWR_LIMIT), // must be last
];