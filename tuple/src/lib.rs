use num_traits::cast::ToPrimitive;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

pub fn eq_with_eps(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

impl Tuple {
    pub fn is_vector(&self) -> Result<bool, &'static str> {
        match self.w {
            w if eq_with_eps(w, 0.0) => Ok(true),
            w if eq_with_eps(w, 1.0) => Ok(false),
            _ => Err("invalid w value"),
        }
    }

    pub fn is_point(&self) -> Result<bool, &'static str> {
        match self.w {
            w if eq_with_eps(w, 1.0) => Ok(true),
            w if eq_with_eps(w, 0.0) => Ok(false),
            _ => Err("invalid w value"),
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_z(&self) -> f64 {
        self.z
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl<T: ToPrimitive> Mul<T> for Tuple {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self {
            x: self.x * rhs.to_f64().unwrap(),
            y: self.y * rhs.to_f64().unwrap(),
            z: self.z * rhs.to_f64().unwrap(),
            w: self.w * rhs.to_f64().unwrap(),
        }
    }
}

impl<T: ToPrimitive> Div<T> for Tuple {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self {
            x: self.x / rhs.to_f64().unwrap(),
            y: self.y / rhs.to_f64().unwrap(),
            z: self.z / rhs.to_f64().unwrap(),
            w: self.w / rhs.to_f64().unwrap(),
        }
    }
}

pub fn tuple<T: ToPrimitive, U: ToPrimitive, V: ToPrimitive>(x: T, y: U, z: V, w: i8) -> Tuple {
    Tuple {
        x: x.to_f64().unwrap(),
        y: y.to_f64().unwrap(),
        z: z.to_f64().unwrap(),
        w: w.to_f64().unwrap(),
    }
}

pub fn point<T: ToPrimitive, U: ToPrimitive, V: ToPrimitive>(x: T, y: U, z: V) -> Tuple {
    Tuple {
        x: x.to_f64().unwrap(),
        y: y.to_f64().unwrap(),
        z: z.to_f64().unwrap(),
        w: 1.0,
    }
}

pub fn vector<T: ToPrimitive, U: ToPrimitive, V: ToPrimitive>(x: T, y: U, z: V) -> Tuple {
    Tuple {
        x: x.to_f64().unwrap(),
        y: y.to_f64().unwrap(),
        z: z.to_f64().unwrap(),
        w: 0.0,
    }
}

pub fn magnitude(v: &Tuple) -> f64 {
    (v.x.powi(2) + v.y.powi(2) + v.z.powi(2) + v.w.powi(2)).sqrt()
}

pub fn normalize(v: &Tuple) -> Tuple {
    Tuple {
        x: v.x / magnitude(&v),
        y: v.y / magnitude(&v),
        z: v.z / magnitude(&v),
        w: v.w / magnitude(&v),
    }
}

pub fn dot(a: &Tuple, b: &Tuple) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

pub fn cross(a: &Tuple, b: &Tuple) -> Tuple {
    vector(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_point() {
        let t: Tuple = tuple(4.3, -4.2, 3.1, 1);
        assert!(eq_with_eps(t.get_x(), 4.3));
        assert!(eq_with_eps(t.get_y(), -4.2));
        assert!(eq_with_eps(t.get_z(), 3.1));
        assert!(t.is_point().unwrap());
        assert!(!t.is_vector().unwrap());
    }

    #[test]
    fn tuple_is_vector() {
        let t: Tuple = tuple(4.3, -4.2, 3.1, 0);
        assert!(eq_with_eps(t.get_x(), 4.3));
        assert!(eq_with_eps(t.get_y(), -4.2));
        assert!(eq_with_eps(t.get_z(), 3.1));
        assert!(!t.is_point().unwrap());
        assert!(t.is_vector().unwrap());
    }

    #[test]
    fn point_creates_tuple_with_w_one() {
        assert_eq!(tuple(4.0, -4.0, 3.0, 1), point(4.0, -4.0, 3.0));
    }

    #[test]
    fn point_creates_tuple_with_w_zero() {
        assert_eq!(tuple(4, -4, 3.0, 0), vector(4.0, -4.0, 3.0));
    }

    #[test]
    fn add_tuples() {
        let a1 = tuple(3.0, -2, 5, 1);
        let a2 = tuple(-2, 3, 1, 0);
        assert_eq!(tuple(1, 1, 6, 1), a1 + a2);
    }

    #[test]
    fn subtract_two_points() {
        let p1 = point(3, 2, 1);
        let p2 = point(5, 6, 7);
        assert_eq!(vector(-2, -4, -6), p1 - p2);
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = point(3, 2, 1);
        let v = vector(5, 6, 7);
        assert_eq!(point(-2, -4, -6), p - v);
    }

    #[test]
    fn subtract_two_vectors() {
        let v1 = vector(3, 2, 1);
        let v2 = vector(5, 6, 7);
        assert_eq!(vector(-2, -4, -6), v1 - v2);
    }

    #[test]
    fn subtract_vector_from_zero_vector() {
        let zero = vector(0, 0, 0);
        let v = vector(1, -2, 3);
        assert_eq!(vector(-1, 2, -3), zero - v);
    }

    #[test]
    fn negating_tuple() {
        let a = tuple(1, -2, 3, -4);
        assert_eq!(tuple(-1, 2, -3, 4), -a);
    }

    #[test]
    fn multiple_tuple_by_scalar() {
        let a = tuple(1, -2, 3, -4);
        assert_eq!(tuple(3.5, -7, 10.5, -14), a * 3.5);
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let a = tuple(1, -2, 3, -4);
        assert_eq!(tuple(0.5, -1, 1.5, -2), a * 0.5);
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let a = tuple(1, -2, 3, -4);
        assert_eq!(tuple(0.5, -1, 1.5, -2), a / 2);
    }

    #[test]
    fn magnitude_of_vector() {
        let v = vector(1, 0, 0);
        assert!(eq_with_eps(1.0, magnitude(&v)));

        let v = vector(0, 1, 0);
        assert!(eq_with_eps(1.0, magnitude(&v)));

        let v = vector(0, 0, 1);
        assert!(eq_with_eps(1.0, magnitude(&v)));

        let v = vector(1, 2, 3);
        assert!(eq_with_eps(14.0_f64.sqrt(), magnitude(&v)));

        let v = vector(-1, -2, -3);
        assert!(eq_with_eps(14.0_f64.sqrt(), magnitude(&v)));
    }

    #[test]
    fn normalize_vector() {
        let v = vector(4, 0, 0);
        assert_eq!(normalize(&v), vector(1, 0, 0));

        let v = vector(1, 2, 3);
        assert_eq!(
            normalize(&v),
            vector(
                1.0_f64 / (14.0_f64).sqrt(),
                2_f64 / (14.0_f64).sqrt(),
                3_f64 / (14.0_f64).sqrt()
            )
        );
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = vector(1, 2, 3);
        let norm = normalize(&v);
        assert!(eq_with_eps(1.0, magnitude(&norm)));
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);
        assert!(eq_with_eps(20.0_f64, dot(&a, &b)));
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);
        assert_eq!(vector(-1, 2, -1), cross(&a, &b));
        assert_eq!(vector(1, -2, 1), cross(&b, &a));
    }
}
