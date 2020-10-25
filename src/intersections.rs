use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::{dot, point};
use crate::utils::eq_with_eps;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug)]
pub struct Intersection<'a, T>
where
    T: Copy + Shape,
{
    pub t: f64,
    pub object: &'a T,
}

impl<T> Intersection<'_, T>
where
    T: Copy + Shape,
{
    fn key(&self) -> u64 {
        unsafe { std::mem::transmute(self.t) }
    }
}

impl<T> PartialEq for Intersection<'_, T>
where
    T: Copy + Shape,
{
    fn eq(&self, other: &Self) -> bool {
        eq_with_eps(self.t, other.t)
    }
}

impl<T> Hash for Intersection<'_, T>
where
    T: Copy + Shape,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key().hash(state);
    }
}

impl<T> Eq for Intersection<'_, T> where T: Copy + Shape {}

pub struct Intersections<'a, T>(pub HashSet<Intersection<'a, T>>)
where
    T: Copy + Shape;

pub fn intersect<'a, T>(object: &'a T, ray: &Ray) -> Intersections<'a, T>
where
    T: Copy + Shape,
{
    // Vector from the sphere's center to the ray origin
    let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);

    let a = dot(&ray.direction, &ray.direction);
    let b = 2.0 * dot(&ray.direction, &sphere_to_ray);
    let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;
    let discriminant = (b * b) - (4.0 * a * c);

    if !eq_with_eps(discriminant, 0.0) && discriminant < 0.0 {
        let set: HashSet<Intersection<T>> = HashSet::new();
        Intersections(set)
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);
        let mut set: HashSet<Intersection<T>> = HashSet::with_capacity(2);
        set.insert(Intersection { t: t1, object });
        set.insert(Intersection { t: t2, object });
        Intersections(set)
    }
}

pub fn hit<'a, T>(intersections: &'a Intersections<T>) -> Option<&'a Intersection<'a, T>>
where
    T: Copy + Shape,
{
    for intersection in &intersections.0 {
        if intersection.t > 0.0 || eq_with_eps(intersection.t, 0.0) {
            return Some(intersection);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;
    use uuid::Uuid;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere { id: Uuid::new_v4() };
        let i = Intersection { t: 3.5, object: &s };
        assert!(eq_with_eps(3.5, i.t));
        assert_eq!(&s, i.object);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere { id: Uuid::new_v4() };
        let i1 = Intersection { t: 1.0, object: &s };
        let i2 = Intersection { t: 2.0, object: &s };
        let mut set: HashSet<Intersection<Sphere>> = HashSet::with_capacity(2);
        set.insert(i1);
        set.insert(i2);
        let xs = Intersections(set);
        assert_eq!(2, xs.0.len());
        assert!(eq_with_eps(1.0, xs.0.get(&i1).unwrap().t));
        assert!(eq_with_eps(2.0, xs.0.get(&i2).unwrap().t));
    }

    // #[test]
    // fn hit_when_all_intersections_have_positive_t() {
    //     let s = Sphere { id: Uuid::new_v4() };
    //     let i1 = Intersection { t: 1.0, object: &s };
    //     let i2 = Intersection { t: 2.0, object: &s };
    //     let xs = Intersections(vec![i2, i1]);
    //     let i = hit(&xs);
    //     assert_eq!(Some(i1), i);
    // }

    // #[test]
    // fn hit_when_some_intersections_have_negative_t() {
    //     let s = Sphere { id: Uuid::new_v4() };
    //     let i1 = Intersection {
    //         t: -1.0,
    //         object: &s,
    //     };
    //     let i2 = Intersection { t: 2.0, object: &s };
    //     let xs = Intersections(vec![i2, i1]);
    //     let i = hit(&xs);
    //     assert_eq!(Some(i2), i);
    // }

    // #[test]
    // fn hit_when_all_intersections_have_negative_t() {
    //     let s = Sphere { id: Uuid::new_v4() };
    //     let i1 = Intersection {
    //         t: -2.0,
    //         object: &s,
    //     };
    //     let i2 = Intersection {
    //         t: -1.0,
    //         object: &s,
    //     };
    //     let xs = Intersections(vec![i2, i1]);
    //     let i = hit(&xs);
    //     assert_eq!(None, i);
    // }

    // #[test]
    // fn hit_when_is_always_the_lowest_nonnegative_intersection() {
    //     let s = Sphere { id: Uuid::new_v4() };
    //     let i1 = Intersection { t: 5.0, object: &s };
    //     let i2 = Intersection { t: 7.0, object: &s };
    //     let i2 = Intersection {
    //         t: -3.0,
    //         object: &s,
    //     };
    //     let i2 = Intersection { t: 2.0, object: &s };
    //     let xs = Intersections(vec![i1, i2, i3, i4]);
    //     let i = hit(&xs);
    //     assert_eq!(i4, i);
    // }
}
