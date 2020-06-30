pub fn eq_with_eps(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}
