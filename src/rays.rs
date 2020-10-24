use crate::tuple::*;

struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    fn position(&self, time: f64) -> Tuple {
        self.origin + self.direction * time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
