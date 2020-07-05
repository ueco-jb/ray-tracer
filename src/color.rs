use crate::utils::eq_with_eps;
use num_traits::cast::ToPrimitive;
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new<T: ToPrimitive, U: ToPrimitive, V: ToPrimitive>(r: T, g: U, b: V) -> Color {
        Color {
            red: r.to_f64().unwrap(),
            green: g.to_f64().unwrap(),
            blue: b.to_f64().unwrap(),
        }
    }

    pub fn get_red(&self) -> f64 {
        self.red
    }

    pub fn get_green(&self) -> f64 {
        self.green
    }

    pub fn get_blue(&self) -> f64 {
        self.blue
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        eq_with_eps(self.red, other.red)
            && eq_with_eps(self.green, other.green)
            && eq_with_eps(self.blue, other.blue)
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

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colors() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert!(eq_with_eps(-0.5_f64, c.red));
        assert!(eq_with_eps(0.4_f64, c.green));
        assert!(eq_with_eps(1.7_f64, c.blue));
    }

    #[test]
    fn add_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(Color::new(1.6, 0.7, 1.0), c1 + c2);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(Color::new(0.2, 0.5, 0.5), c1 - c2);
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(Color::new(0.4, 0.6, 0.8), c * 2);
    }

    #[test]
    fn multiplying_color_by_color() {
        let c1 = Color::new(1, 0.2, 0.4);
        let c2 = Color::new(0.9, 1, 0.1);
        assert_eq!(Color::new(0.9, 0.2, 0.04), c1 * c2);
    }
}
