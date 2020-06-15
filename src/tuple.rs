use num_traits::cast::ToPrimitive;
use std::ops::{Add, Neg, Sub};

#[derive(Debug, PartialEq)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: i8,
}

impl Tuple {
    pub fn is_vector(&self) -> Result<bool, &'static str> {
        match self.w {
            0 => Ok(true),
            1 => Ok(false),
            _ => Err("invalid w value"),
        }
    }

    pub fn is_point(&self) -> Result<bool, &'static str> {
        match self.w {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err("invalid w value"),
        }
    }

    pub fn get_x(&self) -> &f64 {
        &self.x
    }
    pub fn get_y(&self) -> &f64 {
        &self.y
    }
    pub fn get_z(&self) -> &f64 {
        &self.z
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

pub fn tuple<T: ToPrimitive, U: ToPrimitive, V: ToPrimitive>(x: T, y: U, z: V, w: i8) -> Tuple {
    Tuple {
        x: x.to_f64().unwrap(),
        y: y.to_f64().unwrap(),
        z: z.to_f64().unwrap(),
        w,
    }
}

pub fn point<T: ToPrimitive, U: ToPrimitive, V: ToPrimitive>(x: T, y: U, z: V) -> Tuple {
    Tuple {
        x: x.to_f64().unwrap(),
        y: y.to_f64().unwrap(),
        z: z.to_f64().unwrap(),
        w: 1,
    }
}

pub fn vector<T: ToPrimitive, U: ToPrimitive, V: ToPrimitive>(x: T, y: U, z: V) -> Tuple {
    Tuple {
        x: x.to_f64().unwrap(),
        y: y.to_f64().unwrap(),
        z: z.to_f64().unwrap(),
        w: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_point() {
        let t: Tuple = tuple(4.3, -4.2, 3.1, 1);
        assert_eq!(&4.3, t.get_x());
        assert_eq!(&-4.2, t.get_y());
        assert_eq!(&3.1, t.get_z());
        assert!(t.is_point().unwrap());
        assert!(!t.is_vector().unwrap());
    }

    #[test]
    fn tuple_is_vector() {
        let t: Tuple = tuple(4.3, -4.2, 3.1, 0);
        assert_eq!(&4.3, t.get_x());
        assert_eq!(&-4.2, t.get_y());
        assert_eq!(&3.1, t.get_z());
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
}
