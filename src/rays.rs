use crate::sphere::Sphere;
use crate::tuple::*;
use crate::utils::eq_with_eps;

#[allow(dead_code)]
struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    #[allow(dead_code)]
    fn position(&self, time: f64) -> Tuple {
        self.origin + self.direction * time
    }
}

#[allow(dead_code)]
fn intersect(_sphere: &Sphere, ray: &Ray) -> Vec<f64> {
    // Vector from the sphere's center to the ray origin
    let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);

    let a = dot(&ray.direction, &ray.direction);
    let b = 2.0 * dot(&ray.direction, &sphere_to_ray);
    let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;
    let discriminant = (b * b) - (4.0 * a * c);

    if !eq_with_eps(discriminant, 0.0) && discriminant < 0.0 {
        vec![]
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);
        vec![t1, t2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

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
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere { id: Uuid::new_v4() };
        let xs = intersect(&s, &r);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0]);
        assert_eq!(6.0, xs[1]);
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
        assert_eq!(5.0, xs[0]);
        assert_eq!(5.0, xs[1]);
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
        assert_eq!(-1.0, xs[0]);
        assert_eq!(1.0, xs[1]);
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
        assert_eq!(-6.0, xs[0]);
        assert_eq!(-4.0, xs[1]);
    }
}
