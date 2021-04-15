use crate::{
    color::{Color, BLACK},
    intersections::{intersect, Computations, Intersections},
    light::PointLight,
    material::{lighting, Material},
    matrix::MatrixError,
    ray::Ray,
    shape::Shape,
    sphere::Sphere,
    transformations::scaling,
    tuple::point,
};
use std::rc::Rc;

pub struct World {
    pub light: Option<PointLight>,
    pub objects: Vec<Rc<dyn Shape>>,
}

impl Default for World {
    fn default() -> Self {
        let mut s1 = Sphere::default();
        s1.set_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
        });
        let mut s2 = Sphere::default();
        s2.set_transform(scaling(0.5, 0.5, 0.5));
        World {
            light: Some(PointLight {
                position: point(-10.0, -10.0, -10.0),
                intensity: Color::new(1.0, 1.0, 1.0),
            }),
            objects: vec![Rc::new(s1), Rc::new(s2)],
        }
    }
}

impl World {
    #[allow(dead_code)]
    fn new() -> Self {
        World {
            light: None,
            objects: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn get_rco(&self, index: usize) -> Option<&Rc<dyn Shape>> {
        self.objects.get(index)
    }

    #[allow(dead_code)]
    pub fn get_mut_rco(&mut self, index: usize) -> Option<&mut Rc<dyn Shape>> {
        self.objects.get_mut(index)
    }

    pub fn shade_hit(&self, comps: Computations) -> Option<Color> {
        if let Some(light) = self.light {
            Some(lighting(
                comps.object.borrow().get_material(),
                light,
                comps.point,
                comps.eyev,
                comps.normalv,
            ))
        } else {
            None
        }
    }

    fn intersect_world(
        &mut self,
        ray: &Ray,
        intersections: &mut Intersections,
    ) -> Result<(), MatrixError> {
        for o in self.objects.iter() {
            let mut intersection = intersect(o.clone(), ray)?;
            intersections.append(&mut intersection);
        }
        //intersections.extend(world.objects.iter().cloned().map(|o| intersect(o, ray).unwrap()));
        intersections.sort();
        Ok(())
    }

    #[allow(dead_code)]
    pub fn color_at(&mut self, ray: &Ray) -> Result<Color, MatrixError> {
        let mut intersections = Intersections::new();
        self.intersect_world(ray, &mut intersections)?;
        if let Some(intersection) = intersections.hit() {
            let comps = Computations::prepare_computation(intersection.clone(), *ray)?;
            Ok(self.shade_hit(comps).unwrap_or(BLACK))
        } else {
            Ok(BLACK)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{intersections::Intersection, tuple::vector, utils::eq_with_eps};
    use std::{cell::RefCell, rc::Rc};

    // #[test]
    // fn creating_world() {
    //     let w = World::new();
    //     let v: Vec<Rc<Sphere>> = Vec::new();
    //     assert_eq!(None, w.light);
    //     assert_eq!(v, w.objects);
    // }

    #[test]
    fn default_world() {
        let w = World::default();
        let light = PointLight {
            position: point(-10.0, -10.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let mut s1 = Sphere::default();
        s1.set_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
        });
        let mut s2 = Sphere::default();
        s2.set_transform(scaling(0.5, 0.5, 0.5));

        assert_eq!(Some(light), w.light);
        assert!(w
            .objects
            .iter()
            .any(|i| i.get_transform() == s1.get_transform()
                && i.get_material() == s1.get_material()));
        assert!(w
            .objects
            .iter()
            .any(|i| i.get_transform() == s2.get_transform()
                && i.get_material() == s2.get_material()));
    }

    #[test]
    fn intersecting_world_with_ray() {
        let mut w = World::default();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let mut i = Intersections::new();
        w.intersect_world(&r, &mut i).unwrap();
        assert_eq!(4, (*i).len());
        assert!(eq_with_eps(4.0, (*i)[0].t));
        assert!(eq_with_eps(4.5, (*i)[1].t));
        assert!(eq_with_eps(5.5, (*i)[2].t));
        assert!(eq_with_eps(6.0, (*i)[3].t));
    }

    #[test]
    fn shading_intersection() {
        let w = World::default();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape: Sphere = *w.objects[0].as_any().downcast_ref::<Sphere>().unwrap();
        let i = Intersection {
            t: 4.0,
            object: RefCell::new(Rc::new(shape)),
        };
        let comps = Computations::prepare_computation(i, r).unwrap();
        let c = w.shade_hit(comps).unwrap();
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn shading_intersection_from_inside() {
        let light = Some(PointLight {
            position: point(0.0, 0.25, 0.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        });
        let w = World {
            light,
            ..Default::default()
        };
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape: Sphere = *w.objects[1].as_any().downcast_ref::<Sphere>().unwrap();
        let i = Intersection {
            t: 0.5,
            object: RefCell::new(Rc::new(shape)),
        };
        let comps = Computations::prepare_computation(i, r).unwrap();
        let c = w.shade_hit(comps).unwrap();
        assert_eq!(Color::new(00.90498, 0.90498, 0.90498), c);
    }

    #[test]
    fn color_when_ray_misses() {
        let mut w = World::default();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 1.0, 0.0),
        };
        let c = w.color_at(&r).unwrap();
        assert_eq!(BLACK, c);
    }

    #[test]
    fn color_when_ray_hits() {
        let mut w = World::default();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let c = w.color_at(&r).unwrap();
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = World::default();
        let o = w.get_mut_rco(0).unwrap();
        Rc::get_mut(o).unwrap().set_ambient(1.0);
        let inner = w.get_mut_rco(1).unwrap();
        Rc::get_mut(inner).unwrap().set_ambient(1.0);
        let output_color = *inner.get_color();
        let r = Ray {
            origin: point(0.0, 0.0, 0.75),
            direction: vector(0.0, 0.0, -1.0),
        };
        let c = w.color_at(&r).unwrap();
        assert_eq!(output_color, c);
    }
}
