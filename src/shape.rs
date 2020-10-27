use crate::matrix::Matrix4;

pub trait Shape {
    fn get_transform(&self) -> Matrix4;
    fn set_transform(&mut self, transform: Matrix4);
}
