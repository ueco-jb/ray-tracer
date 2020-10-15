use crate::matrix::*;

fn translation(x: f64, y: f64, z: f64) -> Result<Matrix4, MatrixError> {
    let mut m = Matrix4::identity_matrix();
    m.set(0, 3, x)?;
    m.set(1, 3, y)?;
    m.set(2, 3, z)?;
    Ok(m)
}

fn scaling(x: f64, y: f64, z: f64) -> Result<Matrix4, MatrixError> {
    let mut m = Matrix4::identity_matrix();
    m.set(0, 0, x)?;
    m.set(1, 1, y)?;
    m.set(2, 2, z)?;
    Ok(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::*;

    #[test]
    fn multiply_by_translation_matrix() {
        let transform: Matrix4 = translation(5.0, -3.0, 2.0).unwrap();
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(point(2.0, 1.0, 7.0), transform * p);
    }

    #[test]
    fn multiply_by_inverse_of_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0).unwrap();
        let inv = transform.inverse().unwrap();
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(point(-8.0, 7.0, 3.0), inv * p);
    }

    #[test]
    fn translation_doesnt_affect_vector() {
        let transform = translation(5.0, -3.0, 2.0).unwrap();
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
}
