#[derive(Debug)]
pub enum MatrixError {
    OutOfMatrixBorder,
}

pub struct Matrix4 {
    matrix: [[f64; 4]; 4],
}

impl Matrix4 {
    pub fn get_matrix(&self) -> [[f64; 4]; 4] {
        self.matrix
    }

    pub fn set(&mut self, row: usize, column: usize, value: f64) -> Result<(), MatrixError> {
        if row >= 4 || column >= 4 {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            self.matrix[row][column] = value;
            Ok(())
        }
    }

    pub fn get(&self, row: usize, column: usize) -> Result<f64, MatrixError> {
        if row >= 4 || column >= 4 {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            Ok(self.matrix[row][column])
        }
    }
}

pub fn matrix4() -> Matrix4 {
    Matrix4 {
        matrix: [[0.0f64; 4]; 4],
    }
}

#[allow(clippy::too_many_arguments)]
pub fn matrix4_values(
    v1: f64,
    v2: f64,
    v3: f64,
    v4: f64,
    v5: f64,
    v6: f64,
    v7: f64,
    v8: f64,
    v9: f64,
    v10: f64,
    v11: f64,
    v12: f64,
    v13: f64,
    v14: f64,
    v15: f64,
    v16: f64,
) -> Matrix4 {
    let mut m = Matrix4 {
        matrix: [[0.0f64; 4]; 4],
    };
    m.matrix[0][0] = v1;
    m.matrix[0][1] = v2;
    m.matrix[0][2] = v3;
    m.matrix[0][3] = v4;
    m.matrix[1][0] = v5;
    m.matrix[1][1] = v6;
    m.matrix[1][2] = v7;
    m.matrix[1][3] = v8;
    m.matrix[2][0] = v9;
    m.matrix[2][1] = v10;
    m.matrix[2][2] = v11;
    m.matrix[2][3] = v12;
    m.matrix[3][0] = v13;
    m.matrix[3][1] = v14;
    m.matrix[3][2] = v15;
    m.matrix[3][3] = v16;
    m
}

pub struct Matrix3 {
    matrix: [[f64; 3]; 3],
}

impl Matrix3 {
    pub fn get_matrix(&self) -> [[f64; 3]; 3] {
        self.matrix
    }

    pub fn set(&mut self, row: usize, column: usize, value: f64) -> Result<(), MatrixError> {
        if row >= 3 || column >= 3 {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            self.matrix[row][column] = value;
            Ok(())
        }
    }

    pub fn get(&self, row: usize, column: usize) -> Result<f64, MatrixError> {
        if row >= 3 || column >= 3 {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            Ok(self.matrix[row][column])
        }
    }
}

pub fn matrix3() -> Matrix3 {
    Matrix3 {
        matrix: [[0.0f64; 3]; 3],
    }
}

#[allow(clippy::too_many_arguments)]
pub fn matrix3_values(
    v1: f64,
    v2: f64,
    v3: f64,
    v4: f64,
    v5: f64,
    v6: f64,
    v7: f64,
    v8: f64,
    v9: f64,
) -> Matrix3 {
    let mut m = Matrix3 {
        matrix: [[0.0f64; 3]; 3],
    };
    m.matrix[0][0] = v1;
    m.matrix[0][1] = v2;
    m.matrix[0][2] = v3;
    m.matrix[1][0] = v4;
    m.matrix[1][1] = v5;
    m.matrix[1][2] = v6;
    m.matrix[2][0] = v7;
    m.matrix[2][1] = v8;
    m.matrix[2][2] = v9;
    m
}

pub struct Matrix2 {
    matrix: [[f64; 2]; 2],
}

impl Matrix2 {
    pub fn get_matrix(&self) -> [[f64; 2]; 2] {
        self.matrix
    }

    pub fn set(&mut self, row: usize, column: usize, value: f64) -> Result<(), MatrixError> {
        if row >= 2 || column >= 2 {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            self.matrix[row][column] = value;
            Ok(())
        }
    }

    pub fn get(&self, row: usize, column: usize) -> Result<f64, MatrixError> {
        if row >= 2 || column >= 2 {
            Err(MatrixError::OutOfMatrixBorder)
        } else {
            Ok(self.matrix[row][column])
        }
    }
}

pub fn matrix2() -> Matrix2 {
    Matrix2 {
        matrix: [[0.0f64; 2]; 2],
    }
}

pub fn matrix2_values(v1: f64, v2: f64, v3: f64, v4: f64) -> Matrix2 {
    let mut m = Matrix2 {
        matrix: [[0.0f64; 2]; 2],
    };
    m.matrix[0][0] = v1;
    m.matrix[0][1] = v2;
    m.matrix[1][0] = v3;
    m.matrix[1][1] = v4;
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::eq_with_eps;

    #[test]
    fn constructing_4_matrix() {
        let m = matrix4();
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
        let m = matrix4_values(
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
        let m = matrix3();
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
        let m = matrix3_values(-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0);
        assert!(eq_with_eps(-3.0, m.get(0, 0).unwrap()));
        assert!(eq_with_eps(-2.0, m.get(1, 1).unwrap()));
        assert!(eq_with_eps(1.0, m.get(2, 2).unwrap()));
    }

    #[test]
    fn constructing_2_matrix() {
        let m = matrix2();
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
        let m = matrix2_values(-3.0, 5.0, 1.0, -2.0);
        assert!(eq_with_eps(-3.0, m.get(0, 0).unwrap()));
        assert!(eq_with_eps(5.0, m.get(0, 1).unwrap()));
        assert!(eq_with_eps(1.0, m.get(1, 0).unwrap()));
        assert!(eq_with_eps(-2.0, m.get(1, 1).unwrap()));
    }
}
