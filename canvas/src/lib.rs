#![allow(dead_code)]

use num_traits::cast::ToPrimitive;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq)]
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

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl<T: ToPrimitive> Mul<T> for Color {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self {
            red: self.red * rhs.to_f64().unwrap(),
            green: self.green * rhs.to_f64().unwrap(),
            blue: self.blue * rhs.to_f64().unwrap(),
        }
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

    #[test]
    fn add_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(color(1.6, 0.7, 1.0), c1 + c2);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(color(0.2, 0.5, 0.5), c1 - c2);
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = color(0.2, 0.3, 0.4);
        assert_eq!(color(0.4, 0.6, 0.8), c * 2);
    }
}
