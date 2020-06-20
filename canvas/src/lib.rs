#![allow(dead_code)]

use num_traits::cast::ToPrimitive;

pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

pub fn color<T: ToPrimitive, U: ToPrimitive, V: ToPrimitive>(r: T, g: U, b: V) -> Color {
    Color {
        red: r.to_f64().unwrap(),
        green: g.to_f64().unwrap(),
        blue: b.to_f64().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tuple::eq_with_eps;

    #[test]
    fn test_colors() {
        let c = color(-0.5, 0.4, 1.7);
        assert!(eq_with_eps(-0.5_f64, c.red));
        assert!(eq_with_eps(0.4_f64, c.green));
        assert!(eq_with_eps(1.7_f64, c.blue));
    }
}
