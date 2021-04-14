use crate::{
    color::Color,
    material::Material,
    matrix::{Matrix4, MatrixError},
    tuple::Tuple,
};
use std::{any::Any, fmt};
use uuid::Uuid;

#[derive(Debug)]
pub enum ShapeType {
    Sphere,
}

pub trait Shape {
    fn as_any(&self) -> &dyn Any;
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

impl fmt::Debug for dyn Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Shape")
            .field("id", &self.get_id())
            .field("transform", &self.get_transform())
            .field("material", self.get_material())
            .field("color", self.get_color())
            .finish()
    }
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &dyn Shape) -> bool {
        self.get_id() == other.get_id()
    }
}
