use crate::{
    tuple::{Tuple, TupleT},
    utils::eq_with_eps,
};
use std::{
    collections::HashSet,
    ops::{Deref, DerefMut, Mul},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MatrixError {
    #[error("Read or write out of matrix border")]
    OutOfMatrixBorder,
    #[error("Matrix is not invertible")]
    MatrixNotInvertible,
    #[error("Matrix 2x2 cannot have a submatrix")]
    No2x2Submatrix,
}

type Result<T> = std::result::Result<T, MatrixError>;

pub trait Matrix {
    const SIZE: usize;
    type Submatrix: Matrix;

    fn set(&mut self, row: usize, column: usize, value: f64) -> Result<()>;
    fn get(&self, row: usize, column: usize) -> Result<f64>;

    fn boundry_check(&self, row: &usize, column: &usize) -> Result<()> {
        if row >= &Self::SIZE || column >= &Self::SIZE {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            Ok(())
        }
    }

    fn calculate_submatrix_remove_indexes(&self, row: usize, column: usize) -> HashSet<usize> {
        let mut position_to_remove = HashSet::with_capacity(Self::SIZE + 1);
        position_to_remove.insert(row * Self::SIZE);
        position_to_remove.insert(row * Self::SIZE + 1);
        position_to_remove.insert(row * Self::SIZE + 2);
        position_to_remove.insert(column);
        position_to_remove.insert(column + Self::SIZE);
        position_to_remove.insert(column + Self::SIZE * 2);
        if Self::SIZE == 4 {
            position_to_remove.insert(row * Self::SIZE + 3);
            position_to_remove.insert(column + Self::SIZE * 3);
        }
        position_to_remove
    }

    fn submatrix(&self, row: usize, column: usize) -> Result<Self::Submatrix>;

    fn cofactor(&self, row: usize, column: usize) -> Result<f64> {
        match self.boundry_check(&row, &column) {
            Ok(_) => {
                let d = self.submatrix(row, column)?.determiant()?;
                if (row + column) % 2 == 0 {
                    Ok(d)
                } else {
                    Ok(d * -1.0)
                }
            }
            Err(e) => Err(e),
        }
    }

    fn determiant(&self) -> Result<f64> {
        let mut d = 0.0f64;
        for c in 0..Self::SIZE {
            d += self.get(0, c)? * self.cofactor(0, c)?;
        }
        Ok(d)
    }

    fn is_invertible(&self) -> Result<bool> {
        Ok(!eq_with_eps(self.determiant()?, 0.0))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Matrix4(pub [f64; 16]);

impl Deref for Matrix4 {
    type Target = [f64; 16];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Matrix4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Matrix for Matrix4 {
    const SIZE: usize = 4;
    type Submatrix = Matrix3;

    fn set(&mut self, row: usize, column: usize, value: f64) -> Result<()> {
        match self.boundry_check(&row, &column) {
            Ok(_) => {
                (*self)[row * Self::SIZE + column] = value;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn get(&self, row: usize, column: usize) -> Result<f64> {
        match self.boundry_check(&row, &column) {
            Ok(_) => Ok(self.0[row * Self::SIZE + column]),
            Err(e) => Err(e),
        }
    }

    fn submatrix(&self, row: usize, column: usize) -> Result<Self::Submatrix> {
        match self.boundry_check(&row, &column) {
            Ok(_) => {
                let to_remove = self.calculate_submatrix_remove_indexes(row, column);
                let mut submatrix: Self::Submatrix = Default::default();
                let mut i = 0;
                for (index, elem) in self.0.iter().enumerate() {
                    if !to_remove.contains(&index) {
                        submatrix.0[i] = *elem;
                        i += 1;
                    }
                }
                Ok(submatrix)
            }
            Err(e) => Err(e),
        }
    }
}

impl Matrix4 {
    const SIZE: usize = 4;

    pub fn identity_matrix() -> Matrix4 {
        Matrix4([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn transpose(&self) -> Result<Matrix4> {
        let mut output: Self = Default::default();
        for i in 0..Self::SIZE {
            for j in 0..Self::SIZE {
                output.set(j, i, self.get(i, j)?)?;
            }
        }
        Ok(output)
    }

    pub fn inverse(&self) -> Result<Matrix4> {
        if !self.is_invertible()? {
            Err(MatrixError::MatrixNotInvertible)
        } else {
            let d = self.determiant()?;
            let mut m: Self = Default::default();
            for row in 0..Self::SIZE {
                for col in 0..Self::SIZE {
                    let c = self.cofactor(row, col)?;
                    m.set(col, row, c / d)?
                }
            }
            Ok(m)
        }
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().eq_by(&other.0, |&x, &y| eq_with_eps(x, y))
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut m = Self([0.0f64; 16]);
        for i in 0..Self::SIZE {
            let pos_a = i * Self::SIZE;
            let pos_b = i * Self::SIZE + 1;
            let pos_c = i * Self::SIZE + 2;
            let pos_d = i * Self::SIZE + 3;
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

impl Deref for Matrix3 {
    type Target = [f64; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Matrix3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Matrix for Matrix3 {
    const SIZE: usize = 3;
    type Submatrix = Matrix2;

    fn set(&mut self, row: usize, column: usize, value: f64) -> Result<()> {
        match self.boundry_check(&row, &column) {
            Ok(_) => {
                self.0[row * Self::SIZE + column] = value;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn get(&self, row: usize, column: usize) -> Result<f64> {
        match self.boundry_check(&row, &column) {
            Ok(_) => Ok(self.0[row * Self::SIZE + column]),
            Err(e) => Err(e),
        }
    }

    fn submatrix(&self, row: usize, column: usize) -> Result<Self::Submatrix> {
        match self.boundry_check(&row, &column) {
            Ok(_) => {
                let to_remove = self.calculate_submatrix_remove_indexes(row, column);
                let mut submatrix: Self::Submatrix = Default::default();
                let mut i = 0;
                for (index, elem) in self.0.iter().enumerate() {
                    if !to_remove.contains(&index) {
                        submatrix.0[i] = *elem;
                        i += 1;
                    }
                }
                Ok(submatrix)
            }
            Err(e) => Err(e),
        }
    }
}

impl Matrix3 {
    pub fn minor(&self, row: usize, column: usize) -> Result<f64> {
        match self.boundry_check(&row, &column) {
            Ok(_) => Ok(self.submatrix(row, column)?.determiant()?),
            Err(e) => Err(e),
        }
    }
}

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().eq_by(&other.0, |&x, &y| eq_with_eps(x, y))
    }
}

#[derive(Debug, Default)]
pub struct Matrix2([f64; 4]);

impl Deref for Matrix2 {
    type Target = [f64; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Matrix2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Matrix for Matrix2 {
    const SIZE: usize = 2;
    type Submatrix = Matrix2;

    fn set(&mut self, row: usize, column: usize, value: f64) -> Result<()> {
        match self.boundry_check(&row, &column) {
            Ok(_) => {
                self.0[row * Self::SIZE + column] = value;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn get(&self, row: usize, column: usize) -> Result<f64> {
        match self.boundry_check(&row, &column) {
            Ok(_) => Ok(self.0[row * Self::SIZE + column]),
            Err(e) => Err(e),
        }
    }

    fn submatrix(&self, _row: usize, _column: usize) -> Result<Self::Submatrix> {
        Err(MatrixError::No2x2Submatrix)
    }

    fn determiant(&self) -> Result<f64> {
        Ok(self.0[0] * self.0[3] - self.0[1] * self.0[2])
    }
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().eq_by(&other.0, |&x, &y| eq_with_eps(x, y))
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
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        assert_eq!(Tuple::new(18.0, 24.0, 33.0, 1.0), a * b);
    }

    #[test]
    fn multiply_matrix_by_identity_matrix() {
        let a = Matrix4([
            0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
        ]);
        let i = Matrix4::identity_matrix();
        assert_eq!(a, a * i);
    }

    #[test]
    fn multiply_identity_matrix_by_tuple() {
        let i = Matrix4::identity_matrix();
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
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
    fn determiant_of_2x2_matrix() {
        let a = Matrix2([1.0, 5.0, -3.0, 2.0]);
        assert!(eq_with_eps(a.determiant().unwrap(), 17.0));
    }

    #[test]
    fn submatrix_of_3x3_is_2x2() {
        let a = Matrix3([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
        let sub_a = Matrix2([-3.0, 2.0, 0.0, 6.0]);
        assert_eq!(a.submatrix(0, 2).unwrap(), sub_a);
    }

    #[test]
    fn submatrix_of_4x4_is_3x3() {
        let a = Matrix4([
            -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
        ]);
        let sub_a = Matrix3([-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0]);
        assert_eq!(a.submatrix(2, 1).unwrap(), sub_a);
    }

    #[test]
    fn minor_of_3x3_matrix() {
        let a = Matrix3([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        let b = a.submatrix(1, 0).unwrap();
        assert!(eq_with_eps(b.determiant().unwrap(), 25.0));
        assert!(eq_with_eps(a.minor(1, 0).unwrap(), 25.0));
    }

    #[test]
    fn cofactor_of_3x3_matrix() {
        let a = Matrix3([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        assert!(eq_with_eps(a.minor(0, 0).unwrap(), -12.0));
        assert!(eq_with_eps(a.cofactor(0, 0).unwrap(), -12.0));
        assert!(eq_with_eps(a.minor(1, 0).unwrap(), 25.0));
        assert!(eq_with_eps(a.cofactor(1, 0).unwrap(), -25.0));
    }

    #[test]
    fn determiant_of_3x3_matrix() {
        let a = Matrix3([1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);
        assert!(eq_with_eps(a.cofactor(0, 0).unwrap(), 56.0));
        assert!(eq_with_eps(a.cofactor(0, 1).unwrap(), 12.0));
        assert!(eq_with_eps(a.cofactor(0, 2).unwrap(), -46.0));
        assert!(eq_with_eps(a.determiant().unwrap(), -196.0));
    }

    #[test]
    fn determiant_of_4x4_matrix() {
        let a = Matrix4([
            -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
        ]);
        assert!(eq_with_eps(a.cofactor(0, 0).unwrap(), 690.0));
        assert!(eq_with_eps(a.cofactor(0, 1).unwrap(), 447.0));
        assert!(eq_with_eps(a.cofactor(0, 2).unwrap(), 210.0));
        assert!(eq_with_eps(a.cofactor(0, 3).unwrap(), 51.0));
        assert!(eq_with_eps(a.determiant().unwrap(), -4071.0));
    }

    #[test]
    fn is_matrix_invertible() {
        let a = Matrix4([
            6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
        ]);
        assert!(eq_with_eps(a.determiant().unwrap(), -2120.0));
        assert!(a.is_invertible().unwrap());

        let a = Matrix4([
            -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
        ]);
        assert!(eq_with_eps(a.determiant().unwrap(), 0.0));
        assert!(!a.is_invertible().unwrap());
    }

    #[test]
    fn inverse_matrix() {
        let a = Matrix4([
            -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0,
        ]);
        let b = a.inverse().unwrap();
        assert!(eq_with_eps(a.determiant().unwrap(), 532.0));
        assert!(eq_with_eps(a.cofactor(2, 3).unwrap(), -160.0));
        assert!(eq_with_eps(b.get(3, 2).unwrap(), (-160.0) / 532.0));
        assert!(eq_with_eps(a.cofactor(3, 2).unwrap(), 105.0));
        assert!(eq_with_eps(b.get(2, 3).unwrap(), 105.0 / 532.0));
        assert_eq!(
            b,
            Matrix4([
                0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068,
                -0.07895, -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639
            ])
        );
    }

    #[test]
    fn multiply_product_by_its_inverse() {
        let a = Matrix4([
            3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0,
        ]);
        let b = Matrix4([
            8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
        ]);
        let c = a * b;
        assert_eq!(c * b.inverse().unwrap(), a);
    }
}
