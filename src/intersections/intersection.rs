use crate::{
    matrix::MatrixError,
    ray::{transform, Ray},
    shape::Shape,
    tuple::{dot, point},
    utils::eq_with_eps,
};
use std::{
    boxed::Box,
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

type RcRefCellBox<T> = Rc<RefCell<Box<T>>>;

#[derive(Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: RcRefCellBox<dyn Shape>,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        eq_with_eps(self.t, other.t)
            && (*self.object).borrow().get_id() == (*other.object).borrow().get_id()
    }
}

#[repr(transparent)]
pub struct Intersections(Vec<Intersection>);

impl Deref for Intersections {
    type Target = Vec<Intersection>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Intersections {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Intersections {
    pub fn new() -> Intersections {
        Intersections(Vec::new())
    }

    pub fn add(&mut self, elem: Intersection) {
        (*self).push(elem);
    }

    pub fn sort(&mut self) -> &Self {
        (*self).sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Less));
        self
    }

    pub fn hit(&mut self) -> Option<&Intersection> {
        self.sort();
        for intersection in &self.0 {
            if intersection.t > 0.0 || eq_with_eps(intersection.t, 0.0) {
                return Some(intersection);
            }
        }
        None
    }
}

impl Default for Intersections {
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
pub fn intersect(object: Box<dyn Shape>, ray: &Ray) -> Result<Intersections, MatrixError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::default();
        let i = Intersection {
            t: 3.5,
            object: Rc::new(RefCell::new(Box::new(s))),
        };
        assert!(eq_with_eps(3.5, i.t));
        assert_eq!(s.get_id(), (*i.object).borrow().get_id());
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::default();
        let i1 = Intersection {
            t: 1.0,
            object: Rc::new(RefCell::new(Box::new(s))),
        };
        let i2 = Intersection {
            t: 2.0,
            object: Rc::new(RefCell::new(Box::new(s))),
        };
        let xs = Intersections(vec![i1, i2]);
        assert_eq!(2, xs.0.len());
        assert!(eq_with_eps(1.0, xs.0[0].t));
        assert!(eq_with_eps(2.0, xs.0[1].t));
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection {
            t: 1.0,
            object: Rc::new(RefCell::new(Box::new(s))),
        };
        let i2 = Intersection {
            t: 2.0,
            object: Rc::new(RefCell::new(Box::new(s))),
        };
        let mut xs = Intersections(vec![i2, i1.clone()]);
        let i = xs.hit();
        assert_eq!(&i1, i.unwrap());
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection {
            t: -1.0,
            object: Rc::new(RefCell::new(Box::new(s))),
        };
        let i2 = Intersection {
            t: 2.0,
            object: Rc::new(RefCell::new(Box::new(s))),
        };
        let mut xs = Intersections(vec![i2.clone(), i1]);
        let i = xs.hit();
        assert_eq!(&i2, i.unwrap());
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection {
            t: -2.0,
            object: Rc::new(RefCell::new(Box::new(s))),
        };
        let i2 = Intersection {
            t: -1.0,
            object: Rc::new(RefCell::new(Box::new(s))),
        };
        let mut xs = Intersections(vec![i2, i1]);
        let i = xs.hit();
        assert_eq!(None, i);
    }

    #[test]
    fn hit_when_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = Intersection {
            t: 5.0,
            object: Rc::new(RefCell::new(Box::new(s))),
        };
        let i2 = Intersection {
            t: 7.0,
            object: Rc::new(RefCell::new(Box::new(s))),
        };
        let i3 = Intersection {
            t: -3.0,
            object: Rc::new(RefCell::new(Box::new(s))),
        };
        let i4 = Intersection {
            t: 2.0,
            object: Rc::new(RefCell::new(Box::new(s))),
        };
        let mut xs = Intersections(vec![i1, i2, i3, i4.clone()]);
        let i = xs.hit();
        assert_eq!(&i4, i.unwrap());
    }
}
