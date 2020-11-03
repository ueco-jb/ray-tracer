use crate::color::Color;
use crate::material::Material;
use crate::matrix::{Matrix4, MatrixError};
use crate::tuple::Tuple;

pub trait Shape {
    fn get_transform(&self) -> Matrix4;
    fn set_transform(&mut self, transform: Matrix4);
    fn normal_at(&self, p: Tuple) -> Result<Tuple, MatrixError>;
    fn set_material(&mut self, m: Material);
    fn get_material(&mut self) -> &Material;
    fn set_color(&mut self, c: Color);
}
