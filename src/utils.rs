pub fn eq_with_eps(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.00001
}
