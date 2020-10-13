use crate::matrix::*;
use crate::tuple::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::eq_with_eps;

    #[test]
    fn multiply_by_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq(point(2.0, 1.0, 7.0), transform * point);
    }

    #[test]
    fn multiply_by_inverse_of_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = point(-3.0, 4.0, 5.0);
        assert_eq(point(-8.0, 7.0, 3.0), inv * point);
    }

    #[test]
    fn translation_doesnt_affect_vector() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);
        assert_eq(v, transform * v);
    }
