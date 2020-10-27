use crate::matrix::Matrix4;
use crate::shape::Shape;
use uuid::Uuid;

// For simplicity, Sphere currently has radius 1 and center on (0, 0, 0)
#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    id: Uuid,
    transform: Matrix4,
}

impl Shape for Sphere {
    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn get_transform(&self) -> Matrix4 {
        self.transform
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            id: Uuid::new_v4(),
            transform: Matrix4::identity_matrix(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intersections::intersect;
    use crate::ray::Ray;
    use crate::transformations::{scaling, translation};
    use crate::tuple::{point, vector};
    use crate::utils::eq_with_eps;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s: Sphere = Default::default();
        let xs = intersect(&s, &r).unwrap();
        assert_eq!(2, xs.0.len());
        assert!(eq_with_eps(4.0, xs.0[0].t));
        assert!(eq_with_eps(6.0, xs.0[1].t));
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray {
            origin: point(0.0, 1.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s: Sphere = Default::default();
        let xs = intersect(&s, &r).unwrap();
        assert_eq!(2, xs.0.len());
        assert!(eq_with_eps(5.0, xs.0[0].t));
        assert!(eq_with_eps(5.0, xs.0[1].t));
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray {
            origin: point(0.0, 2.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s: Sphere = Default::default();
        let xs = intersect(&s, &r).unwrap();
        assert_eq!(0, xs.0.len());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s: Sphere = Default::default();
        let xs = intersect(&s, &r).unwrap();
        assert_eq!(2, xs.0.len());
        assert!(eq_with_eps(-1.0, xs.0[0].t));
        assert!(eq_with_eps(1.0, xs.0[1].t));
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray {
            origin: point(0.0, 0.0, 5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s: Sphere = Default::default();
        let xs = intersect(&s, &r).unwrap();
        assert_eq!(2, xs.0.len());
        assert!(eq_with_eps(-6.0, xs.0[0].t));
        assert!(eq_with_eps(-4.0, xs.0[1].t));
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s: Sphere = Default::default();
        let xs = intersect(&s, &r).unwrap();
        assert_eq!(2, xs.0.len());
        assert_eq!(&s, xs.0[0].object);
        assert_eq!(&s, xs.0[1].object);
    }

    #[test]
    fn spheres_default_transformation() {
        let s: Sphere = Default::default();
        assert_eq!(Matrix4::identity_matrix(), s.transform);
    }

    #[test]
    fn changing_spheres_transformation() {
        let mut s: Sphere = Default::default();
        let t = translation(2.0, 3.0, 4.0);
        s.set_transform(t);
        assert_eq!(t, s.transform);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let mut s: Sphere = Default::default();
        s.set_transform(scaling(2.0, 2.0, 2.0));
        let xs = intersect(&s, &r).unwrap();
        assert_eq!(2, xs.0.len());
        assert!(eq_with_eps(3.0, xs.0[0].t));
        assert!(eq_with_eps(7.0, xs.0[1].t));
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let mut s: Sphere = Default::default();
        s.set_transform(translation(5.0, 0.0, 0.0));
        let xs = intersect(&s, &r).unwrap();
        assert_eq!(0, xs.0.len());
    }
}
