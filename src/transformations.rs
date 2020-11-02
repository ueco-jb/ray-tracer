use crate::matrix::*;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
    Matrix4([
        1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
    ])
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4 {
    Matrix4([
        x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
    ])
}

pub fn rotation_x(r: f64) -> Matrix4 {
    Matrix4([
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        r.cos(),
        -r.sin(),
        0.0,
        0.0,
        r.sin(),
        r.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ])
}

pub fn rotation_y(r: f64) -> Matrix4 {
    Matrix4([
        r.cos(),
        0.0,
        r.sin(),
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        -r.sin(),
        0.0,
        r.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ])
}

pub fn rotation_z(r: f64) -> Matrix4 {
    Matrix4([
        r.cos(),
        -r.sin(),
        0.0,
        0.0,
        r.sin(),
        r.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ])
}

pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix4 {
    Matrix4([
        1.0, x_y, x_z, 0.0, y_x, 1.0, y_z, 0.0, z_x, z_y, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::*;
    use crate::utils::PI;

    #[test]
    fn multiply_by_translation_matrix() {
        let transform: Matrix4 = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(point(2.0, 1.0, 7.0), transform * p);
    }

    #[test]
    fn multiply_by_inverse_of_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(point(-8.0, 7.0, 3.0), inv * p);
    }

    #[test]
    fn translation_doesnt_affect_vector() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);
        assert_eq!(v, transform * v);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        assert_eq!(point(-8.0, 18.0, 32.0), transform * p);
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(vector(-8.0, 18.0, 32.0), transform * v);
    }

    #[test]
    fn multiplying_by_inverse_of_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(vector(-2.0, 2.0, 2.0), inv * v);
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(point(-2.0, 3.0, 4.0), transform * p);
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);
        assert_eq!(
            point(0.0, 2f64.sqrt() / 2.0f64, 2f64.sqrt() / 2.0f64),
            half_quarter * p
        );
        assert_eq!(point(0.0, 0.0, 1.0), full_quarter * p);
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_opposite_direction() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse().unwrap();
        assert_eq!(
            point(0.0, 2f64.sqrt() / 2.0f64, -(2f64.sqrt() / 2.0f64)),
            inv * p
        );
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);
        assert_eq!(
            point(2f64.sqrt() / 2.0f64, 0.0, 2f64.sqrt() / 2.0f64),
            half_quarter * p
        );
        assert_eq!(point(1.0, 0.0, 0.0), full_quarter * p);
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);
        assert_eq!(
            point(-(2f64.sqrt()) / 2.0f64, 2f64.sqrt() / 2.0f64, 0.0),
            half_quarter * p
        );
        assert_eq!(point(-1.0, 0.0, 0.0), full_quarter * p);
    }

    #[test]
    fn shearing_transformation_moves_parameter_in_proportion_to_other() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(point(5.0, 3.0, 4.0), transform * p);

        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(point(6.0, 3.0, 4.0), transform * p);

        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(point(2.0, 5.0, 4.0), transform * p);

        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(point(2.0, 7.0, 4.0), transform * p);

        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(point(2.0, 3.0, 6.0), transform * p);

        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(point(2.0, 3.0, 7.0), transform * p);
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let p2 = a * p;
        assert_eq!(point(1.0, -1.0, 0.0), p2);
        let p3 = b * p2;
        assert_eq!(point(5.0, -5.0, 0.0), p3);
        let p4 = c * p3;
        assert_eq!(point(15.0, 0.0, 7.0), p4);
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let trans = c * b * a;
        assert_eq!(point(15.0, 0.0, 7.0), p * trans);
    }
}
