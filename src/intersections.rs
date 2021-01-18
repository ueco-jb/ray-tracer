use crate::{
    matrix::MatrixError,
    ray::{transform, Ray},
    shape::Shape,
    tuple::{dot, point, Tuple},
    utils::eq_with_eps,
};
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

#[derive(Clone, Debug)]
pub struct Intersection<T>
where
    T: Shape,
{
    pub t: f64,
    pub object: Rc<RefCell<T>>,
}

impl<T> PartialEq for Intersection<T>
where
    T: Shape + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        eq_with_eps(self.t, other.t) && self.object == other.object
    }
}

#[repr(transparent)]
pub struct Intersections<T: Shape>(Vec<Intersection<T>>);

impl<T: Shape> Deref for Intersections<T> {
    type Target = Vec<Intersection<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Shape> DerefMut for Intersections<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// impl<'a, T: Shape> IntoIterator for &Intersections<'a, T> {
//     type Item = Intersection<'a, T>;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

impl<T> Intersections<T>
where
    T: Shape,
{
    pub fn new() -> Intersections<T> {
        Intersections(Vec::new())
    }

    pub fn add(&mut self, elem: Intersection<T>) {
        (*self).push(elem);
    }

    pub fn sort(&mut self) -> &Self {
        (*self).sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Less));
        self
    }

    pub fn hit(&mut self) -> Option<&Intersection<T>>
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

impl<T: Shape> Default for Intersections<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Logic behind scaling is that instead of actually turning/rotating object, you move point
/// (origin of ray) as inverse to operation
/// In example instead of making sphere two times bigger, you shrink the distance between ray and
/// the sphere
/// In order to calculate proper intersection on scaled object, you need to apply inverse of
/// sphere's transformation onto ray
pub fn intersect<T>(object: T, ray: &Ray) -> Result<Intersections<T>, MatrixError>
where
    T: Shape,
{
    let ray2 = transform(*ray, object.get_transform().inverse()?);
    // Vector from the sphere's center to the ray origin
    let sphere_to_ray = ray2.origin - point(0.0, 0.0, 0.0);
    let a = dot(&ray2.direction, &ray2.direction);
    let b = 2.0 * dot(&ray2.direction, &sphere_to_ray);
    let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;
    let discriminant = (b * b) - (4.0 * a * c);

    if !eq_with_eps(discriminant, 0.0) && discriminant < 0.0 {
        Ok(Intersections(vec![]))
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);
        let rc_object = Rc::new(RefCell::new(object));
        Ok(Intersections(vec![
            Intersection {
                t: t1,
                object: Rc::clone(&rc_object),
            },
            Intersection {
                t: t2,
                object: Rc::clone(&rc_object),
            },
        ]))
    }
}

pub struct Computations<T>
where
    T: Shape,
{
    pub t: f64,
    pub object: Rc<RefCell<T>>,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl<T: Shape> Computations<T> {
    fn is_inside(normalv: &Tuple, eyev: &Tuple) -> bool {
        dot(normalv, eyev) < 0.0
    }

    pub fn prepare_computation(
        intersection: Intersection<T>,
        ray: Ray,
    ) -> Result<Computations<T>, MatrixError> {
        let t = intersection.t;
        let ray_position = ray.position(t);
        let eyev = -ray.direction;
        let mut normalv = (*intersection.object).borrow().normal_at(ray_position)?;
        let inside = Self::is_inside(&eyev, &normalv);
        if inside {
            normalv = -normalv;
        };
        Ok(Computations {
            t,
            object: Rc::clone(&intersection.object),
            point: ray_position,
            eyev,
            normalv,
            inside,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        sphere::Sphere,
        tuple::{point, vector},
    };

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s: Sphere = Default::default();
        let i = Intersection {
            t: 3.5,
            object: Rc::new(RefCell::new(s)),
        };
        assert!(eq_with_eps(3.5, i.t));
        assert_eq!(s, (*(*i.object).borrow()));
    }

    #[test]
    fn aggregating_intersections() {
        let s: Sphere = Default::default();
        let i1 = Intersection {
            t: 1.0,
            object: Rc::new(RefCell::new(s)),
        };
        let i2 = Intersection {
            t: 2.0,
            object: Rc::new(RefCell::new(s)),
        };
        let xs = Intersections(vec![i1, i2]);
        assert_eq!(2, xs.0.len());
        assert!(eq_with_eps(1.0, xs.0[0].t));
        assert!(eq_with_eps(2.0, xs.0[1].t));
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s: Sphere = Default::default();
        let i1 = Intersection {
            t: 1.0,
            object: Rc::new(RefCell::new(s)),
        };
        let i2 = Intersection {
            t: 2.0,
            object: Rc::new(RefCell::new(s)),
        };
        let mut xs = Intersections(vec![i2, i1.clone()]);
        let i = xs.hit();
        assert_eq!(&i1, i.unwrap());
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s: Sphere = Default::default();
        let i1 = Intersection {
            t: -1.0,
            object: Rc::new(RefCell::new(s)),
        };
        let i2 = Intersection {
            t: 2.0,
            object: Rc::new(RefCell::new(s)),
        };
        let mut xs = Intersections(vec![i2.clone(), i1]);
        let i = xs.hit();
        assert_eq!(&i2, i.unwrap());
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s: Sphere = Default::default();
        let i1 = Intersection {
            t: -2.0,
            object: Rc::new(RefCell::new(s)),
        };
        let i2 = Intersection {
            t: -1.0,
            object: Rc::new(RefCell::new(s)),
        };
        let mut xs = Intersections(vec![i2, i1]);
        let i = xs.hit();
        assert_eq!(None, i);
    }

    #[test]
    fn hit_when_is_always_the_lowest_nonnegative_intersection() {
        let s: Sphere = Default::default();
        let i1 = Intersection {
            t: 5.0,
            object: Rc::new(RefCell::new(s)),
        };
        let i2 = Intersection {
            t: 7.0,
            object: Rc::new(RefCell::new(s)),
        };
        let i3 = Intersection {
            t: -3.0,
            object: Rc::new(RefCell::new(s)),
        };
        let i4 = Intersection {
            t: 2.0,
            object: Rc::new(RefCell::new(s)),
        };
        let mut xs = Intersections(vec![i1, i2, i3, i4.clone()]);
        let i = xs.hit();
        assert_eq!(&i4, i.unwrap());
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape: Sphere = Default::default();
        let i = Intersection {
            t: 4.0,
            object: Rc::new(RefCell::new(shape)),
        };
        let comps = Computations::prepare_computation(i.clone(), r).unwrap();
        assert!(eq_with_eps(i.t, comps.t));
        assert_eq!(i.object, comps.object);
        assert_eq!(point(0.0, 0.0, -1.0), comps.point);
        assert_eq!(vector(0.0, 0.0, -1.0), comps.eyev);
        assert_eq!(vector(0.0, 0.0, -1.0), comps.normalv);
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_outside() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape: Sphere = Default::default();
        let i = Intersection {
            t: 4.0,
            object: Rc::new(RefCell::new(shape)),
        };
        let comps = Computations::prepare_computation(i, r).unwrap();
        assert_eq!(false, comps.inside);
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_inside() {
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape: Sphere = Default::default();
        let i = Intersection {
            t: 1.0,
            object: Rc::new(RefCell::new(shape)),
        };
        let comps = Computations::prepare_computation(i, r).unwrap();
        assert_eq!(point(0.0, 0.0, 1.0), comps.point);
        assert_eq!(vector(0.0, 0.0, -1.0), comps.eyev);
        assert_eq!(vector(0.0, 0.0, -1.0), comps.normalv);
        assert_eq!(true, comps.inside);
    }
}
