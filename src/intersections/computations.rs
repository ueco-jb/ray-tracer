use crate::{
    intersections::Intersection,
    matrix::MatrixError,
    ray::Ray,
    shape::Shape,
    tuple::{dot, Tuple},
};
use std::{cell::RefCell, rc::Rc};

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
        utils::eq_with_eps,
    };

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
