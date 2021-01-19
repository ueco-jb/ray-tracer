use crate::{
    color::Color,
    material::Material,
    matrix::{Matrix4, MatrixError},
    tuple::Tuple,
};
use uuid::Uuid;

pub trait Shape {
    fn get_transform(&self) -> Matrix4;
    fn set_transform(&mut self, transform: Matrix4);
    fn normal_at(&self, p: Tuple) -> Result<Tuple, MatrixError>;
    fn set_material(&mut self, m: Material);
    fn get_material(&self) -> &Material;
    fn get_color(&self) -> &Color;
    fn set_color(&mut self, c: Color);
    fn set_ambient(&mut self, a: f64);
    fn get_id(&self) -> &Uuid;
}
