use crate::{
    color::{Color, BLACK},
    light::PointLight,
    tuple::{dot, normalize, reflect, Tuple},
    utils::eq_with_eps,
};

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

/// Calculating reflections using Phong reflection model
pub fn lighting(
    m: &Material,
    light: PointLight,
    position: Tuple,
    eyev: Tuple,
    normalv: Tuple,
) -> Color {
    let mut diffuse = BLACK;
    let mut specular = BLACK;

    // combine the surface color with the light's color/intensity
    let effective_color = m.color * light.intensity;

    // find the direction to the light source
    let lightv = normalize(&(light.position - position));

    // compute the ambient contribution
    let ambient = effective_color * m.ambient;

    // light_dot_normal represents the consine of the angle between the light vector and the
    // normal vector. A negative number means the light is on the other side of the surface
    let light_dot_normal = dot(&lightv, &normalv);
    if light_dot_normal > 0.0 || eq_with_eps(0.0, light_dot_normal) {
        // compute the diffuse contribution
        diffuse = effective_color * m.diffuse * light_dot_normal;

        // reflect_dot_eye representsd the cosine of the angle between the reflection vector
        // and the eye vector. A negative number means the light reflects away from the eye
        let reflectv = reflect(&-lightv, &normalv);
        let reflect_dot_eye = dot(&reflectv, &eyev);

        if reflect_dot_eye > 0.0 {
            // compute the specular contribution
            let factor = reflect_dot_eye.powf(m.shininess);
            specular = light.intensity * m.specular * factor;
        }
    }

    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::{point, vector};

    fn setup() -> (Material, Tuple) {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        (m, position)
    }

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_eq!(Color::new(1.0, 1.0, 1.0), m.color);
        assert!(eq_with_eps(0.1, m.ambient));
        assert!(eq_with_eps(0.9, m.diffuse));
        assert!(eq_with_eps(0.9, m.specular));
        assert!(eq_with_eps(200.0, m.shininess));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let (m, position) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 0.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let result = lighting(&m, light, position, eyev, normalv);
        assert_eq!(Color::new(1.9, 1.9, 1.9), result);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_and_eye_offset_45() {
        let (m, position) = setup();
        let two_sqrt = 2.0f64.sqrt();
        let eyev = vector(0.0, two_sqrt / 2.0, -two_sqrt / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 0.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let result = lighting(&m, light, position, eyev, normalv);
        assert_eq!(Color::new(1.0, 1.0, 1.0), result);
    }

    #[test]
    fn lighting_with_eye_opposite_surface_and_light_offset_45() {
        let (m, position) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 10.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let result = lighting(&m, light, position, eyev, normalv);
        assert_eq!(Color::new(0.7364, 0.7364, 0.7364), result);
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let (m, position) = setup();
        let two_sqrt = 2.0f64.sqrt();
        let eyev = vector(0.0, -two_sqrt / 2.0, -two_sqrt / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 10.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let result = lighting(&m, light, position, eyev, normalv);
        assert_eq!(Color::new(1.6364, 1.6364, 1.6364), result);
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let (m, position) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 0.0, 10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let result = lighting(&m, light, position, eyev, normalv);
        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }
}
