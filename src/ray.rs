use crate::matrix::Matrix4;
use crate::tuple::*;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn position(&self, time: f64) -> Tuple {
        self.origin + self.direction * time
    }
}

fn transform(ray: Ray, matrix: Matrix4) -> Ray {
    Ray {
        origin: ray.origin * matrix,
        direction: ray.direction * matrix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformations::{scaling, translation};

    #[test]
    fn creating_and_querrying_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);

        let r = Ray { origin, direction };

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn compute_point_from_distance() {
        let r = Ray {
            origin: point(2.0, 3.0, 4.0),
            direction: vector(1.0, 0.0, 0.0),
        };
        assert_eq!(point(2.0, 3.0, 4.0), r.position(0.0));
        assert_eq!(point(3.0, 3.0, 4.0), r.position(1.0));
        assert_eq!(point(1.0, 3.0, 4.0), r.position(-1.0));
        assert_eq!(point(4.5, 3.0, 4.0), r.position(2.5));
    }

    #[test]
    fn translate_ray() {
        let r = Ray {
            origin: point(1.0, 2.0, 3.0),
            direction: vector(0.0, 1.0, 0.0),
        };
        let m = translation(3.0, 4.0, 5.0);
        let r2 = transform(r, m);
        assert_eq!(point(4.0, 6.0, 8.0), r2.origin);
        assert_eq!(vector(0.0, 1.0, 0.0), r2.direction);
    }

    #[test]
    fn scaling_ray() {
        let r = Ray {
            origin: point(1.0, 2.0, 3.0),
            direction: vector(0.0, 1.0, 0.0),
        };
        let m = scaling(2.0, 3.0, 4.0);
        let r2 = transform(r, m);
        assert_eq!(point(2.0, 6.0, 12.0), r2.origin);
        assert_eq!(vector(0.0, 3.0, 0.0), r2.direction);
    }
}
