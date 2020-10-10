use crate::tuple::Tuple;
use crate::utils::eq_with_eps;
use std::num::sqrt;
use std::ops::Mul;

#[derive(Debug)]
pub enum MatrixError {
    OutOfMatrixBorder,
    IncorrectMatrixSize(usize),
}

trait Matrix {
    fn set(&mut self, row: usize, column: usize, value: f64) -> Result<(), MatrixError> {
        let length = self.0.len();
        if length != 4 || lengeh != 9 || length != 16 {
            Err(IncorrectMatrixSize(length))
        }
        let len_sqrt = sqrt(length);
        if row >= len_sqrt || len_sqrt >= 4 {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            self.matrix[row * len_sqrt + column] = value;
            Ok(())
        }
    }

    fn get(&self, row: usize, column: usize) -> Result<f64, MatrixError> {
        let length = self.0.len();
        if length != 4 || lengeh != 9 || length != 16 {
            Err(IncorrectMatrixSize(length))
        }
        let len_sqrt = sqrt(length);
        if row >= len_sqrt || len_sqrt >= 4 {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            Ok(self.matrix[row * len_sqrt + column])
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Matrix4([f64; 16]);

impl Matrix for Matrix4 {}

// #[derive(Clone, Debug, Default)]
// pub struct Matrix4 {
//     matrix: [[f64; 4]; 4],
// }
//
// impl Matrix4 {
//     pub fn identity_matrix() -> Matrix4 {
//         Matrix4::new_with_values(
//             1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
//         )
//     }
//
//     #[allow(clippy::too_many_arguments)]
//     pub fn new_with_values(
//         v1: f64,
//         v2: f64,
//         v3: f64,
//         v4: f64,
//         v5: f64,
//         v6: f64,
//         v7: f64,
//         v8: f64,
//         v9: f64,
//         v10: f64,
//         v11: f64,
//         v12: f64,
//         v13: f64,
//         v14: f64,
//         v15: f64,
//         v16: f64,
//     ) -> Matrix4 {
//         let mut m = Matrix4 {
//             matrix: [[0.0f64; 4]; 4],
//         };
//         m.matrix[0][0] = v1;
//         m.matrix[0][1] = v2;
//         m.matrix[0][2] = v3;
//         m.matrix[0][3] = v4;
//         m.matrix[1][0] = v5;
//         m.matrix[1][1] = v6;
//         m.matrix[1][2] = v7;
//         m.matrix[1][3] = v8;
//         m.matrix[2][0] = v9;
//         m.matrix[2][1] = v10;
//         m.matrix[2][2] = v11;
//         m.matrix[2][3] = v12;
//         m.matrix[3][0] = v13;
//         m.matrix[3][1] = v14;
//         m.matrix[3][2] = v15;
//         m.matrix[3][3] = v16;
//         m
//     }
//
//     pub fn get_matrix(&self) -> [[f64; 4]; 4] {
//         self.matrix
//     }
//
//     pub fn set(&mut self, row: usize, column: usize, value: f64) -> Result<(), MatrixError> {
//         if row >= 4 || column >= 4 {
//             Err(MatrixError::OutOfMatrixBorder)
//         } else {
//             self.matrix[row][column] = value;
//             Ok(())
//         }
//     }
//
//     pub fn get(&self, row: usize, column: usize) -> Result<f64, MatrixError> {
//         if row >= 4 || column >= 4 {
//             Err(MatrixError::OutOfMatrixBorder)
//         } else {
//             Ok(self.matrix[row][column])
//         }
//     }
//
//     pub fn transpose(&self) -> Result<Matrix4, MatrixError> {
//         let mut output: Matrix4 = Default::default();
//         for (i, row) in self.matrix.iter().enumerate() {
//             for (j, _col) in row.iter().enumerate() {
//                 output.set(j, i, self.matrix[i][j])?;
//             }
//         }
//         Ok(output)
//     }
// }

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Matrix4) -> bool {
        eq_with_eps(self.0[0], other.0[0])
            && eq_with_eps(self.0[1], other.0[1])
            && eq_with_eps(self.0[2], other.0[2])
            && eq_with_eps(self.0[3], other.0[3])
            && eq_with_eps(self.0[4], other.0[4])
            && eq_with_eps(self.0[5], other.0[5])
            && eq_with_eps(self.0[6], other.0[6])
            && eq_with_eps(self.0[7], other.0[7])
            && eq_with_eps(self.0[8], other.0[8])
            && eq_with_eps(self.0[9], other.0[9])
            && eq_with_eps(self.0[10], other.0[10])
            && eq_with_eps(self.0[11], other.0[11])
            && eq_with_eps(self.0[12], other.0[12])
            && eq_with_eps(self.0[13], other.0[13])
            && eq_with_eps(self.0[14], other.0[14])
            && eq_with_eps(self.0[15], other.0[15])
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    // fn mul(self, rhs: Self) -> Self {
    //     let mut m = Self([0.0f64; 16]);
    //     for (i, row) in self.matrix.iter().enumerate() {
    //         for (j, _col) in row.iter().enumerate() {
    //             m.matrix[i][j] = self.matrix[i][0] * rhs.matrix[0][j]
    //                 + self.matrix[i][1] * rhs.matrix[1][j]
    //                 + self.matrix[i][2] * rhs.matrix[2][j]
    //                 + self.matrix[i][3] * rhs.matrix[3][j];
    //         }
    //     }
    //     m
    // }

    fn mul(self, rhs: Self) -> Self {
        let mut m = Self([0.0f64; 16]);
        for i in 0..4 {
            m.0[i * 4] = self.0[i * 4] * rhs.0[0]
                + self.0[i * 4 + 1] * self.0[4]
                + self.0[i * 4 + 2] * self.0[8]
                + self.0[i * 4 + 3] * self.0[12];
            m.0[i * 4 + 1] = self.0[i * 4] * rhs.0[1]
                + self.0[i * 4 + 1] * self.0[5]
                + self.0[i * 4 + 2] * self.0[9]
                + self.0[i * 4 + 3] * self.0[13];
            m.0[i * 4 + 2] = self.0[i * 4] * rhs.0[2]
                + self.0[i * 4 + 1] * self.0[6]
                + self.0[i * 4 + 2] * self.0[10]
                + self.0[i * 4 + 3] * self.0[14];
            m.0[i * 4 + 3] = self.0[i * 4] * rhs.0[3]
                + self.0[i * 4 + 1] * self.0[7]
                + self.0[i * 4 + 2] * self.0[11]
                + self.0[i * 4 + 3] * self.0[15];
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

// #[derive(Default)]
// pub struct Matrix3 {
//     matrix: [[f64; 3]; 3],
// }
//
// impl Matrix3 {
//     #[allow(clippy::too_many_arguments)]
//     pub fn new_with_values(
//         v1: f64,
//         v2: f64,
//         v3: f64,
//         v4: f64,
//         v5: f64,
//         v6: f64,
//         v7: f64,
//         v8: f64,
//         v9: f64,
//     ) -> Matrix3 {
//         let mut m = Matrix3 {
//             matrix: [[0.0f64; 3]; 3],
//         };
//         m.matrix[0][0] = v1;
//         m.matrix[0][1] = v2;
//         m.matrix[0][2] = v3;
//         m.matrix[1][0] = v4;
//         m.matrix[1][1] = v5;
//         m.matrix[1][2] = v6;
//         m.matrix[2][0] = v7;
//         m.matrix[2][1] = v8;
//         m.matrix[2][2] = v9;
//         m
//     }
//
//     pub fn get_matrix(&self) -> [[f64; 3]; 3] {
//         self.matrix
//     }
//
//     pub fn set(&mut self, row: usize, column: usize, value: f64) -> Result<(), MatrixError> {
//         if row >= 3 || column >= 3 {
//             Err(MatrixError::OutOfMatrixBorder)
//         } else {
//             self.matrix[row][column] = value;
//             Ok(())
//         }
//     }
//
//     pub fn get(&self, row: usize, column: usize) -> Result<f64, MatrixError> {
//         if row >= 3 || column >= 3 {
//             Err(MatrixError::OutOfMatrixBorder)
//         } else {
//             Ok(self.matrix[row][column])
//         }
//     }
// }
//
// #[derive(Default)]
// pub struct Matrix2 {
//     matrix: [[f64; 2]; 2],
// }
//
// impl Matrix2 {
//     pub fn new_with_values(v1: f64, v2: f64, v3: f64, v4: f64) -> Matrix2 {
//         let mut m = Matrix2 {
//             matrix: [[0.0f64; 2]; 2],
//         };
//         m.matrix[0][0] = v1;
//         m.matrix[0][1] = v2;
//         m.matrix[1][0] = v3;
//         m.matrix[1][1] = v4;
//         m
//     }
//     pub fn get_matrix(&self) -> [[f64; 2]; 2] {
//         self.matrix
//     }
//
//     pub fn set(&mut self, row: usize, column: usize, value: f64) -> Result<(), MatrixError> {
//         if row >= 2 || column >= 2 {
//             Err(MatrixError::OutOfMatrixBorder)
//         } else {
//             self.matrix[row][column] = value;
//             Ok(())
//         }
//     }
//
//     pub fn get(&self, row: usize, column: usize) -> Result<f64, MatrixError> {
//         if row >= 2 || column >= 2 {
//             Err(MatrixError::OutOfMatrixBorder)
//         } else {
//             Ok(self.matrix[row][column])
//         }
//     }
// }

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
        let m = Matrix4::new_with_values(
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        );
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
        let m = Matrix3::new_with_values(-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0);
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
        let m = Matrix2::new_with_values(-3.0, 5.0, 1.0, -2.0);
        assert!(eq_with_eps(-3.0, m.get(0, 0).unwrap()));
        assert!(eq_with_eps(5.0, m.get(0, 1).unwrap()));
        assert!(eq_with_eps(1.0, m.get(1, 0).unwrap()));
        assert!(eq_with_eps(-2.0, m.get(1, 1).unwrap()));
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix4::new_with_values(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        let b = Matrix4::new_with_values(
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        );
        assert_eq!(
            Matrix4::new_with_values(
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0
            ),
            a * b
        );
    }

    #[test]
    fn multiply_matrix_by_tuple() {
        let a = Matrix4::new_with_values(
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        );
        let b = Tuple::new(1, 2, 3, 1);
        assert_eq!(Tuple::new(18, 24, 33, 1), a * b);
    }

    #[test]
    fn multiply_matrix_by_identity_matrix() {
        let a = Matrix4::new_with_values(
            0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
        );
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
        let a = Matrix4::new_with_values(
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        );
        assert_eq!(
            Matrix4::new_with_values(
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0
            ),
            a.transpose().unwrap()
        );
    }

    #[test]
    fn transpose_identity_matrix() {
        let a = Matrix4::identity_matrix();
        assert_eq!(a, a.transpose().unwrap());
    }
}
