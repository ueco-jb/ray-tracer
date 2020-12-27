use crate::{
    color::Color, light::PointLight, material::Material, shape::Shape, sphere::Sphere,
    transformations::scaling, tuple::point,
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
