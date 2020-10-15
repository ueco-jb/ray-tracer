pub fn eq_with_eps(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.00001
}

pub fn deg_to_rad(deg: f64) -> f64 {
    (((deg / 180.0f64) * std::f64::consts::PI) * 10000.0).trunc() / 10000.0f64
}
