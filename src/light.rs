use crate::color::Color;
use crate::tuple::Tuple;

pub struct PointLight {
    pub intensity: Color,
    pub position: Tuple,
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
