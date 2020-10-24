use uuid::Uuid;

// For simplicity, Sphere currently has radius 1 and center on (0, 0, 0)
#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub id: Uuid,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intersections::intersect;
    use crate::ray::Ray;
    use crate::tuple::{point, vector};
    use crate::utils::eq_with_eps;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere { id: Uuid::new_v4() };
        let xs = intersect(&s, &r);
        assert_eq!(2, xs.len());
        assert!(eq_with_eps(4.0, xs[0]));
        assert!(eq_with_eps(6.0, xs[1]));
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray {
            origin: point(0.0, 1.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere { id: Uuid::new_v4() };
        let xs = intersect(&s, &r);
        assert_eq!(2, xs.len());
        assert!(eq_with_eps(5.0, xs[0]));
        assert!(eq_with_eps(5.0, xs[1]));
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray {
            origin: point(0.0, 2.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere { id: Uuid::new_v4() };
        let xs = intersect(&s, &r);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere { id: Uuid::new_v4() };
        let xs = intersect(&s, &r);
        assert_eq!(2, xs.len());
        assert!(eq_with_eps(-1.0, xs[0]));
        assert!(eq_with_eps(1.0, xs[1]));
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray {
            origin: point(0.0, 0.0, 5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere { id: Uuid::new_v4() };
        let xs = intersect(&s, &r);
        assert_eq!(2, xs.len());
        assert!(eq_with_eps(-6.0, xs[0]));
        assert!(eq_with_eps(-4.0, xs[1]));
    }
}
