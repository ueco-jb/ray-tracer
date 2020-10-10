use crate::tuple::Tuple;
use crate::utils::eq_with_eps;
use std::ops::Mul;

#[derive(Debug)]
pub enum MatrixError {
    OutOfMatrixBorder,
}

#[derive(Clone, Debug, Default)]
pub struct Matrix4([f64; 16]);

impl Matrix4 {
    pub fn identity_matrix() -> Matrix4 {
        Matrix4([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn set(&mut self, row: usize, column: usize, value: f64) -> Result<(), MatrixError> {
        if row >= 4 || column >= 4 {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            self.0[row * 4 + column] = value;
            Ok(())
        }
    }

    pub fn get(&self, row: usize, column: usize) -> Result<f64, MatrixError> {
        if row >= 4 || column >= 4 {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            Ok(self.0[row * 4 + column])
        }
    }

    pub fn transpose(&self) -> Result<Matrix4, MatrixError> {
        let mut output: Matrix4 = Default::default();
        for i in 0..4 {
            for j in 0..4 {
                output.set(j, i, self.get(i, j)?)?;
            }
        }
        Ok(output)
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Result<Matrix3, MatrixError> {
        if row >= 4 || column >= 4 {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            Ok(self.matrix[row][column])
        }
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Matrix4) -> bool {
        self.0.iter().eq_by(&other.0, |&x, &y| eq_with_eps(x, y))
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut m = Self([0.0f64; 16]);
        for i in 0..4 {
            let pos_a = i * 4;
            let pos_b = i * 4 + 1;
            let pos_c = i * 4 + 2;
            let pos_d = i * 4 + 3;
            m.0[pos_a] = self.0[pos_a] * rhs.0[0]
                + self.0[pos_b] * rhs.0[4]
                + self.0[pos_c] * rhs.0[8]
                + self.0[pos_d] * rhs.0[12];
            m.0[pos_b] = self.0[pos_a] * rhs.0[1]
                + self.0[pos_b] * rhs.0[5]
                + self.0[pos_c] * rhs.0[9]
                + self.0[pos_d] * rhs.0[13];
            m.0[pos_c] = self.0[pos_a] * rhs.0[2]
                + self.0[pos_b] * rhs.0[6]
                + self.0[pos_c] * rhs.0[10]
                + self.0[pos_d] * rhs.0[14];
            m.0[pos_d] = self.0[pos_a] * rhs.0[3]
                + self.0[pos_b] * rhs.0[7]
                + self.0[pos_c] * rhs.0[11]
                + self.0[pos_d] * rhs.0[15];
        }
        m
    }
}

impl Mul<Tuple> for Matrix4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        let v1 = self.0[0] * rhs.get_x()
            + self.0[1] * rhs.get_y()
            + self.0[2] * rhs.get_z()
            + self.0[3] * rhs.get_w();
        let v2 = self.0[4] * rhs.get_x()
            + self.0[5] * rhs.get_y()
            + self.0[6] * rhs.get_z()
            + self.0[7] * rhs.get_w();
        let v3 = self.0[8] * rhs.get_x()
            + self.0[9] * rhs.get_y()
            + self.0[10] * rhs.get_z()
            + self.0[11] * rhs.get_w();
        let v4 = self.0[12] * rhs.get_x()
            + self.0[13] * rhs.get_y()
            + self.0[14] * rhs.get_z()
            + self.0[15] * rhs.get_w();
        Tuple::new(v1, v2, v3, v4)
    }
}

#[derive(Debug, Default)]
pub struct Matrix3([f64; 9]);

impl Matrix3 {
    pub fn set(&mut self, row: usize, column: usize, value: f64) -> Result<(), MatrixError> {
        if row >= 3 || column >= 3 {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            self.0[row * 3 + column] = value;
            Ok(())
        }
    }

    pub fn get(&self, row: usize, column: usize) -> Result<f64, MatrixError> {
        if row >= 3 || column >= 3 {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            Ok(self.0[row * 3 + column])
        }
    }
}

#[derive(Debug, Default)]
pub struct Matrix2([f64; 4]);

impl Matrix2 {
    pub fn set(&mut self, row: usize, column: usize, value: f64) -> Result<(), MatrixError> {
        if row >= 2 || column >= 2 {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            self.0[row * 2 + column] = value;
            Ok(())
        }
    }

    pub fn get(&self, row: usize, column: usize) -> Result<f64, MatrixError> {
        if row >= 2 || column >= 2 {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            Ok(self.0[row * 2 + column])
        }
    }

    pub fn determiant(&self) -> f64 {
        self.matrix[0][0] * self.matrix[1][1] - self.matrix[0][1] * self.matrix[1][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::eq_with_eps;

    #[test]
    fn constructing_4_matrix() {
        let m: Matrix4 = Default::default();
        assert!(eq_with_eps(0.0, m.get(0, 0).unwrap()));
        assert!(eq_with_eps(0.0, m.get(0, 3).unwrap()));
        assert!(eq_with_eps(0.0, m.get(1, 0).unwrap()));
        assert!(eq_with_eps(0.0, m.get(1, 2).unwrap()));
        assert!(eq_with_eps(0.0, m.get(3, 1).unwrap()));
        assert!(eq_with_eps(0.0, m.get(3, 3).unwrap()));

        assert!(m.get(4, 4).is_err());
        assert!(m.get(0, 4).is_err());
        assert!(m.get(4, 0).is_err());
    }

    #[test]
    fn constructing_with_values_4_matrix() {
        let m = Matrix4([
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        ]);
        assert!(eq_with_eps(1.0, m.get(0, 0).unwrap()));
        assert!(eq_with_eps(4.0, m.get(0, 3).unwrap()));
        assert!(eq_with_eps(5.5, m.get(1, 0).unwrap()));
        assert!(eq_with_eps(7.5, m.get(1, 2).unwrap()));
        assert!(eq_with_eps(11.0, m.get(2, 2).unwrap()));
        assert!(eq_with_eps(13.5, m.get(3, 0).unwrap()));
        assert!(eq_with_eps(15.5, m.get(3, 2).unwrap()));
    }

    #[test]
    fn constructing_3_matrix() {
        let m: Matrix3 = Default::default();
        assert!(eq_with_eps(0.0, m.get(0, 0).unwrap()));
        assert!(eq_with_eps(0.0, m.get(0, 2).unwrap()));
        assert!(eq_with_eps(0.0, m.get(1, 0).unwrap()));
        assert!(eq_with_eps(0.0, m.get(1, 2).unwrap()));
        assert!(eq_with_eps(0.0, m.get(2, 1).unwrap()));
        assert!(eq_with_eps(0.0, m.get(2, 2).unwrap()));

        assert!(m.get(3, 3).is_err());
        assert!(m.get(0, 3).is_err());
        assert!(m.get(3, 0).is_err());
    }

    #[test]
    fn constructing_with_values_3_matrix() {
        let m = Matrix3([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);
        assert!(eq_with_eps(-3.0, m.get(0, 0).unwrap()));
        assert!(eq_with_eps(-2.0, m.get(1, 1).unwrap()));
        assert!(eq_with_eps(1.0, m.get(2, 2).unwrap()));
    }

    #[test]
    fn constructing_2_matrix() {
        let m: Matrix2 = Default::default();
        assert!(eq_with_eps(0.0, m.get(0, 0).unwrap()));
        assert!(eq_with_eps(0.0, m.get(0, 1).unwrap()));
        assert!(eq_with_eps(0.0, m.get(1, 0).unwrap()));
        assert!(eq_with_eps(0.0, m.get(1, 1).unwrap()));

        assert!(m.get(2, 2).is_err());
        assert!(m.get(0, 2).is_err());
        assert!(m.get(2, 0).is_err());
    }

    #[test]
    fn constructing_with_values_2_matrix() {
        let m = Matrix2([-3.0, 5.0, 1.0, -2.0]);
        assert!(eq_with_eps(-3.0, m.get(0, 0).unwrap()));
        assert!(eq_with_eps(5.0, m.get(0, 1).unwrap()));
        assert!(eq_with_eps(1.0, m.get(1, 0).unwrap()));
        assert!(eq_with_eps(-2.0, m.get(1, 1).unwrap()));
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix4([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let b = Matrix4([
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        ]);
        assert_eq!(
            Matrix4([
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0
            ]),
            a * b
        );
    }

    #[test]
    fn multiply_matrix_by_tuple() {
        let a = Matrix4([
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ]);
        let b = Tuple::new(1, 2, 3, 1);
        assert_eq!(Tuple::new(18, 24, 33, 1), a * b);
    }

    #[test]
    fn multiply_matrix_by_identity_matrix() {
        let a = Matrix4([
            0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
        ]);
        let i = Matrix4::identity_matrix();
        assert_eq!(a, a.clone() * i);
    }

    #[test]
    fn multiply_identity_matrix_by_tuple() {
        let i = Matrix4::identity_matrix();
        let a = Tuple::new(1, 2, 3, 4);
        assert_eq!(a, i * a);
    }

    #[test]
    fn transpose_matrix() {
        let a = Matrix4([
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        ]);
        assert_eq!(
            Matrix4([
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0
            ]),
            a.transpose().unwrap()
        );
    }

    #[test]
    fn transpose_identity_matrix() {
        let a = Matrix4::identity_matrix();
        assert_eq!(a, a.transpose().unwrap());
    }

    #[test]
    fn calculating_determiant_of_2x2_matrix() {
        let a = Matrix2::new_with_values(1.0, 5.0, -3.0, 2.0);
        assert_eq!(a.determiant(), 17.0);
    }

    #[test]
    fn submatrix_of_3x3_is_2x2() {
        let a = Matrix3::new_with_values(1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0);
        let sub_a = Matrix2::new_with_values(-3.0, 2.0, 0.0, 6.0);
        assert_eq!(a.submatrix(0, 2), sub_a);
    }

    #[test]
    fn submatrix_of_4x4_is_3x3() {
        let a = Matrix4::new_with_values(
            -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
        );
        let sub_a = Matrix3::new_with_values(-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0);
        assert_eq!(a.submatrix(2, 1), sub_a);
    }
}
