use crate::{
    color::Color,
    intersections::{intersect, Intersections},
    light::PointLight,
    material::Material,
    matrix::MatrixError,
    ray::Ray,
    shape::Shape,
    sphere::Sphere,
    transformations::scaling,
    tuple::point,
};

pub struct World {
    pub light: Option<PointLight>,
    pub objects: Vec<Sphere>,
}

impl Default for World {
    fn default() -> Self {
        let mut s1: Sphere = Default::default();
        s1.set_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
        });
        let mut s2: Sphere = Default::default();
        s2.set_transform(scaling(0.5, 0.5, 0.5));
        World {
            light: Some(PointLight {
                position: point(-10.0, -10.0, -10.0),
                intensity: Color::new(1.0, 1.0, 1.0),
            }),
            objects: vec![s1, s2],
        }
    }
}

impl World {
    fn new() -> Self {
        World {
            light: None,
            objects: vec![],
        }
    }
}

fn intersect_world(
    world: &World,
    ray: &Ray,
    intersections: &mut Intersections<Sphere>,
) -> Result<(), MatrixError> {
    for o in world.objects.iter() {
        let mut intersection = intersect(*o, ray)?;
        intersections.append(&mut intersection);
    }
    intersections.sorts();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::vector;

    #[test]
    fn creating_world() {
        let w = World::new();
        let v: Vec<Sphere> = Vec::new();
        assert_eq!(None, w.light);
        assert_eq!(v, w.objects);
    }

    #[test]
    fn default_world() {
        let w: World = Default::default();
        let light = PointLight {
            position: point(-10.0, -10.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let mut s1: Sphere = Default::default();
        s1.set_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
        });
        let mut s2: Sphere = Default::default();
        s2.set_transform(scaling(0.5, 0.5, 0.5));

        assert_eq!(Some(light), w.light);
        assert!(w
            .objects
            .iter()
            .any(|&i| i.get_transform() == s1.get_transform()
                && i.get_material() == s1.get_material()));
        assert!(w
            .objects
            .iter()
            .any(|&i| i.get_transform() == s2.get_transform()
                && i.get_material() == s2.get_material()));
    }

    #[test]
    fn intersecting_world_with_ray() {
        let w: World = Default::default();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let mut i: Intersections<Sphere> = Intersections::new();
        intersect_world(&w, &r, &mut i).unwrap();
        assert_eq!(4, (*i).len());
        assert_eq!(4.0, (*i)[0].t);
        assert_eq!(4.5, (*i)[1].t);
        assert_eq!(5.5, (*i)[2].t);
        assert_eq!(6.0, (*i)[3].t);
    }
}
