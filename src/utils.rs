pub const PI: f64 = std::f64::consts::PI;

pub fn eq_with_eps(a: f64, b: f64) -> bool {
    // This epsilon is "good enough" for my calculations
    (a - b).abs() < 0.00001
}

pub fn deg_to_rad(deg: f64) -> f64 {
    (((deg / 180.0f64) * PI) * 10000.0).trunc() / 10000.0f64
}
