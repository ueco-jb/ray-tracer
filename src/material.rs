use crate::color::Color;
use crate::utils::eq_with_eps;

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Material) -> bool {
        self.color == other.color
            && eq_with_eps(self.ambient, other.ambient)
            && eq_with_eps(self.diffuse, other.diffuse)
            && eq_with_eps(self.specular, other.specular)
            && eq_with_eps(self.shininess, other.shininess)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_material() {
        let m: Material = Default::default();
        assert_eq!(Color::new(1.0, 1.0, 1.0), m.color);
        assert!(eq_with_eps(0.1, m.ambient));
        assert!(eq_with_eps(0.9, m.diffuse));
        assert!(eq_with_eps(0.9, m.specular));
        assert!(eq_with_eps(200.0, m.shininess));
    }
}
