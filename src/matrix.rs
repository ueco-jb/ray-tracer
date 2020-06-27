struct Matrix4 {
    matrix: [[i32;4];4],
}

pub fn matrix4 () -> Matrix4 {
    Matrix4: {
        matrix: [i32;4];4] = [[0i32;4];4];
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_inspecting_4_matrix():
        let matrix = matrix();

