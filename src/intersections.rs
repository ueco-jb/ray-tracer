use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::{dot, point};
use crate::utils::eq_with_eps;

#[allow(dead_code)]
pub struct Intersection<T> {
    pub t: f64,
    pub object: T,
}

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
}
