use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::{dot, point};
use crate::utils::eq_with_eps;

#[derive(Copy, Clone, Debug)]
pub struct Intersection<'a, T>
where
    T: Shape,
{
    pub t: f64,
    pub object: &'a T,
}

impl<'a, T> PartialEq for Intersection<'_, T>
where
    T: Shape + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        eq_with_eps(self.t, other.t) && self.object == other.object
    }
}

pub struct Intersections<'a, T>(pub Vec<Intersection<'a, T>>)
where
    T: Shape;

impl<'a, T> Intersections<'a, T>
where
    T: Shape,
{
    fn sort(&mut self) -> &Self {
        self.0
            .sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Less));
        self
    }

    pub fn hit(&'a mut self) -> Option<&'a Intersection<'a, T>>
    where
        T: Shape,
    {
        self.sort();
        for intersection in &self.0 {
            if intersection.t > 0.0 || eq_with_eps(intersection.t, 0.0) {
                return Some(intersection);
            }
        }
        None
    }
}

pub fn intersect<'a, T>(object: &'a T, ray: &Ray) -> Intersections<'a, T>
where
    T: Shape,
{
    // Vector from the sphere's center to the ray origin
    let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);

    let a = dot(&ray.direction, &ray.direction);
    let b = 2.0 * dot(&ray.direction, &sphere_to_ray);
    let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;
    let discriminant = (b * b) - (4.0 * a * c);

    if !eq_with_eps(discriminant, 0.0) && discriminant < 0.0 {
        Intersections(vec![])
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);
        Intersections(vec![
            Intersection { t: t1, object },
            Intersection { t: t2, object },
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;
    use uuid::Uuid;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection { t: 3.5, object: &s };
        assert!(eq_with_eps(3.5, i.t));
        assert_eq!(&s, i.object);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection { t: 1.0, object: &s };
        let i2 = Intersection { t: 2.0, object: &s };
        let xs = Intersections(vec![i1, i2]);
        assert_eq!(2, xs.0.len());
        assert!(eq_with_eps(1.0, xs.0[0].t));
        assert!(eq_with_eps(2.0, xs.0[1].t));
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection { t: 1.0, object: &s };
        let i2 = Intersection { t: 2.0, object: &s };
        let mut xs = Intersections(vec![i2, i1]);
        let i = xs.hit();
        assert_eq!(&i1, i.unwrap());
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection {
            t: -1.0,
            object: &s,
        };
        let i2 = Intersection { t: 2.0, object: &s };
        let mut xs = Intersections(vec![i2, i1]);
        let i = xs.hit();
        assert_eq!(&i2, i.unwrap());
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection {
            t: -2.0,
            object: &s,
        };
        let i2 = Intersection {
            t: -1.0,
            object: &s,
        };
        let mut xs = Intersections(vec![i2, i1]);
        let i = xs.hit();
        assert_eq!(None, i);
    }

    #[test]
    fn hit_when_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection { t: 5.0, object: &s };
        let i2 = Intersection { t: 7.0, object: &s };
        let i3 = Intersection {
            t: -3.0,
            object: &s,
        };
        let i4 = Intersection { t: 2.0, object: &s };
        let mut xs = Intersections(vec![i1, i2, i3, i4]);
        let i = xs.hit();
        assert_eq!(&i4, i.unwrap());
    }
}
