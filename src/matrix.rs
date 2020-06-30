pub struct Matrix4 {
    matrix: [[f64;4];4],
}

#[derive(Debug)]
pub enum MatrixError {
    OutOfMatrixBorder,
}

impl Matrix4 {
    pub fn get_matrix(&self) -> [[f64;4];4] {
        self.matrix
    }

    pub fn set(&mut self, row: usize, column: usize, value: f64) -> Result<(), MatrixError> {
        self.matrix[row][column] = value;
        Ok(())
    }

    pub fn get(&self, row: usize, column: usize) -> Result<f64, MatrixError> {
        Ok(self.matrix[row][column])
    }
}

pub fn matrix4() -> Matrix4 {
    Matrix4 {
        matrix: [[0.0f64;4];4],
    }
}

pub fn matrix4_values(v1: f64, v2: f64, v3: f64, v4: f64, v5: f64, v6: f64, v7: f64, v8: f64, v9: f64, v10: f64, v11: f64, v12: f64, v13: f64, v14: f64, v15: f64, v16: f64) -> Matrix4 {
    let mut m = Matrix4 {
        matrix: [[0.0f64;4];4],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_4_matrix() {
        let m = matrix4();
        assert_eq!(0.0, m.get(0, 0).unwrap());
        assert_eq!(0.0, m.get(0, 3).unwrap());
        assert_eq!(0.0, m.get(1, 0).unwrap());
        assert_eq!(0.0, m.get(1, 2).unwrap());
        assert_eq!(0.0, m.get(3, 1).unwrap());
        assert_eq!(0.0, m.get(3, 3).unwrap());
    }

    #[test]
    fn constructing_with_values_4_matrix() {
        let m = matrix4_values(1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5);
        assert_eq!(1.0, m.get(0, 0).unwrap());
        assert_eq!(4.0, m.get(0, 3).unwrap());
        assert_eq!(5.5, m.get(1, 0).unwrap());
        assert_eq!(7.5, m.get(1, 2).unwrap());
        assert_eq!(11.0, m.get(2, 2).unwrap());
        assert_eq!(13.5, m.get(3, 0).unwrap());
        assert_eq!(15.5, m.get(3, 2).unwrap());
    }
}

