use crate::matrix::Matrix4;
use crate::utils::eq_with_eps;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait TupleT {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self;

    fn get_x(&self) -> f64;

    fn get_y(&self) -> f64;

    fn get_z(&self) -> f64;

    fn get_w(&self) -> f64;

    fn set_w(&mut self, w: f64);
}

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl TupleT for Tuple {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_y(&self) -> f64 {
        self.y
    }

    fn get_z(&self) -> f64 {
        self.z
    }

    fn get_w(&self) -> f64 {
        self.w
    }

    fn set_w(&mut self, w: f64) {
        self.w = w
    }
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
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        eq_with_eps(self.x, other.x)
            && eq_with_eps(self.y, other.y)
            && eq_with_eps(self.z, other.z)
            && eq_with_eps(self.w, other.w)
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

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<Matrix4> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: Matrix4) -> Tuple {
        let v1 = rhs.0[0] * self.get_x()
            + rhs.0[1] * self.get_y()
            + rhs.0[2] * self.get_z()
            + rhs.0[3] * self.get_w();
        let v2 = rhs.0[4] * self.get_x()
            + rhs.0[5] * self.get_y()
            + rhs.0[6] * self.get_z()
            + rhs.0[7] * self.get_w();
        let v3 = rhs.0[8] * self.get_x()
            + rhs.0[9] * self.get_y()
            + rhs.0[10] * self.get_z()
            + rhs.0[11] * self.get_w();
        let v4 = rhs.0[12] * self.get_x()
            + rhs.0[13] * self.get_y()
            + rhs.0[14] * self.get_z()
            + rhs.0[15] * self.get_w();
        Tuple::new(v1, v2, v3, v4)
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
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

pub fn reflect(incoming: &Tuple, normal: &Tuple) -> Tuple {
    *incoming - *normal * 2.0 * dot(incoming, normal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_point() {
        let t: Tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert!(eq_with_eps(t.get_x(), 4.3));
        assert!(eq_with_eps(t.get_y(), -4.2));
        assert!(eq_with_eps(t.get_z(), 3.1));
        assert!(t.is_point().unwrap());
        assert!(!t.is_vector().unwrap());
    }

    #[test]
    fn tuple_is_vector() {
        let t: Tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert!(eq_with_eps(t.get_x(), 4.3));
        assert!(eq_with_eps(t.get_y(), -4.2));
        assert!(eq_with_eps(t.get_z(), 3.1));
        assert!(!t.is_point().unwrap());
        assert!(t.is_vector().unwrap());
    }

    #[test]
    fn point_creates_tuple_with_w_one() {
        assert_eq!(Tuple::new(4.0, -4.0, 3.0, 1.0), point(4.0, -4.0, 3.0));
    }

    #[test]
    fn point_creates_tuple_with_w_zero() {
        assert_eq!(Tuple::new(4.0, -4.0, 3.0, 0.0), vector(4.0, -4.0, 3.0));
    }

    #[test]
    fn add_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(Tuple::new(1.0, 1.0, 6.0, 1.0), a1 + a2);
    }

    #[test]
    fn subtract_two_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert_eq!(vector(-2.0, -4.0, -6.0), p1 - p2);
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert_eq!(point(-2.0, -4.0, -6.0), p - v);
    }

    #[test]
    fn subtract_two_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        assert_eq!(vector(-2.0, -4.0, -6.0), v1 - v2);
    }

    #[test]
    fn subtract_vector_from_zero_vector() {
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);
        assert_eq!(vector(-1.0, 2.0, -3.0), zero - v);
    }

    #[test]
    fn negating_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(Tuple::new(-1.0, 2.0, -3.0, 4.0), -a);
    }

    #[test]
    fn multiple_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(Tuple::new(3.5, -7.0, 10.5, -14.0), a * 3.5);
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(Tuple::new(0.5, -1.0, 1.5, -2.0), a * 0.5);
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(Tuple::new(0.5, -1.0, 1.5, -2.0), a / 2.0);
    }

    #[test]
    fn magnitude_of_vector() {
        let v = vector(1.0, 0.0, 0.0);
        assert!(eq_with_eps(1.0, magnitude(&v)));

        let v = vector(0.0, 1.0, 0.0);
        assert!(eq_with_eps(1.0, magnitude(&v)));

        let v = vector(0.0, 0.0, 1.0);
        assert!(eq_with_eps(1.0, magnitude(&v)));

        let v = vector(1.0, 2.0, 3.0);
        assert!(eq_with_eps(14.0_f64.sqrt(), magnitude(&v)));

        let v = vector(-1.0, -2.0, -3.0);
        assert!(eq_with_eps(14.0_f64.sqrt(), magnitude(&v)));
    }

    #[test]
    fn normalize_vector() {
        let v = vector(4.0, 0.0, 0.0);
        assert_eq!(normalize(&v), vector(1.0, 0.0, 0.0));

        let v = vector(1.0, 2.0, 3.0);
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
        let v = vector(1.0, 2.0, 3.0);
        let norm = normalize(&v);
        assert!(eq_with_eps(1.0, magnitude(&norm)));
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert!(eq_with_eps(20.0_f64, dot(&a, &b)));
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(vector(-1.0, 2.0, -1.0), cross(&a, &b));
        assert_eq!(vector(1.0, -2.0, 1.0), cross(&b, &a));
    }

    #[test]
    fn reflecting_vector_approaching_at_45() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = reflect(&v, &n);
        assert_eq!(vector(1.0, 1.0, 0.0), r);
    }

    #[test]
    fn reflecting_vector_off_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);
        let two_sqrt = 2.0f64.sqrt();
        let n = vector(two_sqrt / 2.0, two_sqrt / 2.0, 0.0);
        let r = reflect(&v, &n);
        assert_eq!(vector(1.0, 0.0, 0.0), r);
    }
}
