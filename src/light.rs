use crate::{color::Color, tuple::Tuple};

#[derive(Copy, Clone, Debug)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Tuple,
}

impl PartialEq for PointLight {
    fn eq(&self, other: &PointLight) -> bool {
        self.intensity == other.intensity && self.position == other.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::point;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = point(0.0, 0.0, 0.0);
        let light = PointLight {
            position,
            intensity,
        };
        assert_eq!(position, light.position);
        assert_eq!(intensity, light.intensity);
    }
}
