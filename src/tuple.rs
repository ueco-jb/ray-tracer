use num_traits::cast::ToPrimitive;
use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: u8,
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

pub fn tuple<T: ToPrimitive, U: ToPrimitive, V: ToPrimitive>(x: T, y: U, z: V, w: u8) -> Tuple {
    Tuple {
        x: x.to_f64().unwrap(),
        y: y.to_f64().unwrap(),
        z: z.to_f64().unwrap(),
        w,
    }
}


pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x: x,
        y: y,
        z: z,
        w: 1
    }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x: x,
        y: y,
        z: z,
        w: 0
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
    fn add_two_tuples() {
        let a1 = tuple(3.0, -2, 5, 1);
        let a2 = tuple(-2, 3, 1, 0);
        assert_eq!(tuple(1, 1, 6, 1), a1 + a2);
    }
}
