use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::{dot, point};
use crate::utils::eq_with_eps;

#[derive(Copy, Clone)]
pub struct Intersection<T: Copy> {
    t: f64,
    object: T,
}

pub struct Intersections<T: Copy>(Vec<Intersection<T>>);

pub fn intersect(_sphere: &Sphere, ray: &Ray) -> Vec<f64> {
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
    use crate::sphere::Sphere;
    use uuid::Uuid;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere { id: Uuid::new_v4() };
        let i = Intersection { t: 3.5, object: s };
        assert!(eq_with_eps(3.5, i.t));
        assert_eq!(s, i.object);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere { id: Uuid::new_v4() };
        let i1 = Intersection { t: 1.0, object: s };
        let i2 = Intersection { t: 2.0, object: s };
        let xs = Intersections(vec![i1, i2]);
        assert_eq!(2, xs.0.len());
        assert!(eq_with_eps(1.0, xs.0[0].t));
        assert!(eq_with_eps(2.0, xs.0[1].t));
    }
}
