use crate::matrix::*;

const PI: f64 = std::f64::consts::PI;

fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
    Matrix4([
        1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
    ])
}

fn scaling(x: f64, y: f64, z: f64) -> Result<Matrix4, MatrixError> {
    let mut m = Matrix4::identity_matrix();
    m.set(0, 0, x)?;
    m.set(1, 1, y)?;
    m.set(2, 2, z)?;
    Ok(m)
}

fn rotation_x(r: f64) -> Result<Matrix4, MatrixError> {
    let mut m = Matrix4::identity_matrix();
    m.set(1, 1, r.cos())?;
    m.set(1, 2, -r.sin())?;
    m.set(2, 1, r.sin())?;
    m.set(2, 2, r.cos())?;
    Ok(m)
}

fn rotation_y(r: f64) -> Result<Matrix4, MatrixError> {
    let mut m = Matrix4::identity_matrix();
    m.set(0, 0, r.cos())?;
    m.set(0, 2, r.sin())?;
    m.set(2, 0, -r.sin())?;
    m.set(2, 2, r.cos())?;
    Ok(m)
}

fn rotation_z(r: f64) -> Result<Matrix4, MatrixError> {
    let mut m = Matrix4::identity_matrix();
    m.set(0, 0, r.cos())?;
    m.set(0, 1, -r.sin())?;
    m.set(1, 0, r.sin())?;
    m.set(1, 1, r.cos())?;
    Ok(m)
}

fn shearing(
    x_y: f64,
    x_z: f64,
    y_x: f64,
    y_z: f64,
    z_x: f64,
    z_y: f64,
) -> Result<Matrix4, MatrixError> {
    let mut m = Matrix4::identity_matrix();
    m.set(0, 1, x_y)?;
    m.set(0, 2, x_z)?;
    m.set(1, 0, y_x)?;
    m.set(1, 2, y_z)?;
    m.set(2, 0, z_x)?;
    m.set(2, 1, z_y)?;
    Ok(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::*;
    use crate::utils::eq_with_eps;

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
        let transform = scaling(2.0, 3.0, 4.0).unwrap();
        let p = point(-4.0, 6.0, 8.0);
        assert_eq!(point(-8, 18, 32), transform * p);
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = scaling(2.0, 3.0, 4.0).unwrap();
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(vector(-8, 18, 32), transform * v);
    }

    #[test]
    fn multiplying_by_inverse_of_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0).unwrap();
        let inv = transform.inverse().unwrap();
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(vector(-2.0, 2.0, 2.0), inv * v);
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = scaling(-1.0, 1.0, 1.0).unwrap();
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(point(-2.0, 3.0, 4.0), transform * p);
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0).unwrap();
        let full_quarter = rotation_x(PI / 2.0).unwrap();
        assert_eq!(
            point(0.0, 2f64.sqrt() / 2.0f64, 2f64.sqrt() / 2.0f64),
            half_quarter * p
        );
        assert_eq!(point(0.0, 0.0, 1.0), full_quarter * p);
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_opposite_direction() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0).unwrap();
        let inv = half_quarter.inverse().unwrap();
        assert_eq!(
            point(0.0, 2f64.sqrt() / 2.0f64, -(2f64.sqrt() / 2.0f64)),
            inv * p
        );
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0).unwrap();
        let full_quarter = rotation_y(PI / 2.0).unwrap();
        assert_eq!(
            point(2f64.sqrt() / 2.0f64, 0.0, 2f64.sqrt() / 2.0f64),
            half_quarter * p
        );
        assert_eq!(point(1.0, 0.0, 0.0), full_quarter * p);
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0).unwrap();
        let full_quarter = rotation_z(PI / 2.0).unwrap();
        assert_eq!(
            point(-2f64.sqrt() / 2.0f64, 2f64.sqrt() / 2.0f64, 0.0),
            half_quarter * p
        );
        assert_eq!(point(-1.0, 0.0, 0.0), full_quarter * p);
    }

    #[test]
    fn shearing_transformation_moves_parameter_in_proportion_to_other() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0).unwrap();
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(point(5.0, 3.0, 4.0), transform * p);

        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0).unwrap();
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(point(6.0, 3.0, 4.0), transform * p);

        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0).unwrap();
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(point(2.0, 5.0, 4.0), transform * p);

        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(point(2.0, 7.0, 4.0), transform * p);

        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0).unwrap();
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(point(2.0, 3.0, 6.0), transform * p);

        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0).unwrap();
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(point(2.0, 3.0, 7.0), transform * p);
    }
}
