use crate::{
    color::Color,
    material::Material,
    matrix::{Matrix4, MatrixError},
    shape::Shape,
    tuple::{normalize, point, Tuple, TupleT},
};
use std::any::Any;
use uuid::Uuid;

// For simplicity, Sphere currently has radius 1 and center on (0, 0, 0)
#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    id: Uuid,
    transform: Matrix4,
    material: Material,
}

impl Shape for Sphere {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn get_transform(&self) -> Matrix4 {
        self.transform
    }

    /// Normal at point on sphere is a vector perpendicular to the surface - it's the normal
    fn normal_at(&self, world_point: Tuple) -> Result<Tuple, MatrixError> {
        // converting point from world space to object space by multiplying point by inverse of
        // transformation matrix
        let object_point = self.transform.inverse()? * world_point;
        let object_normal = object_point - point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse()?.transpose()? * object_normal;
        // hack - in order to avoid multiplication and inversing a submatrix of transformation,
        // parameter w is set by hand to 0; otherwise some transformation might corrupt that value
        world_normal.set_w(0.0);
        Ok(normalize(&world_normal))
    }

    fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn set_color(&mut self, c: Color) {
        self.material.color = c;
    }

    fn get_color(&self) -> &Color {
        &self.material.color
    }

    fn set_ambient(&mut self, a: f64) {
        self.material.ambient = a;
    }

    fn get_id(&self) -> &Uuid {
        &self.id
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            id: Uuid::new_v4(),
            transform: Matrix4::identity_matrix(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        intersections::intersect,
        ray::Ray,
        transformations::{rotation_z, scaling, translation},
        tuple::vector,
        utils::{eq_with_eps, PI},
    };
    use std::rc::Rc;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::default();
        let xs = intersect(Rc::new(s), &r).unwrap();
        assert_eq!(2, xs.len());
        assert!(eq_with_eps(4.0, xs[0].t));
        assert!(eq_with_eps(6.0, xs[1].t));
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray {
            origin: point(0.0, 1.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::default();
        let xs = intersect(Rc::new(s), &r).unwrap();
        assert_eq!(2, xs.len());
        assert!(eq_with_eps(5.0, xs[0].t));
        assert!(eq_with_eps(5.0, xs[1].t));
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray {
            origin: point(0.0, 2.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::default();
        let xs = intersect(Rc::new(s), &r).unwrap();
        assert_eq!(0, xs.len());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::default();
        let xs = intersect(Rc::new(s), &r).unwrap();
        assert_eq!(2, xs.len());
        assert!(eq_with_eps(-1.0, xs[0].t));
        assert!(eq_with_eps(1.0, xs[1].t));
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray {
            origin: point(0.0, 0.0, 5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::default();
        let xs = intersect(Rc::new(s), &r).unwrap();
        assert_eq!(2, xs.len());
        assert!(eq_with_eps(-6.0, xs[0].t));
        assert!(eq_with_eps(-4.0, xs[1].t));
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::default();
        let xs = intersect(Rc::new(s), &r).unwrap();
        assert_eq!(2, xs.len());
        assert_eq!(s.get_id(), xs[0].object.borrow().get_id());
        assert_eq!(s.get_id(), xs[1].object.borrow().get_id());
    }

    #[test]
    fn spheres_default_transformation() {
        let s = Sphere::default();
        assert_eq!(Matrix4::identity_matrix(), s.transform);
    }

    #[test]
    fn changing_spheres_transformation() {
        let mut s = Sphere::default();
        let t = translation(2.0, 3.0, 4.0);
        s.set_transform(t);
        assert_eq!(t, s.transform);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let mut s = Sphere::default();
        s.set_transform(scaling(2.0, 2.0, 2.0));
        let xs = intersect(Rc::new(s), &r).unwrap();
        assert_eq!(2, xs.len());
        assert!(eq_with_eps(3.0, xs[0].t));
        assert!(eq_with_eps(7.0, xs[1].t));
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let mut s = Sphere::default();
        s.set_transform(translation(5.0, 0.0, 0.0));
        let xs = intersect(Rc::new(s), &r).unwrap();
        assert_eq!(0, xs.len());
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = Sphere::default();
        let n = s.normal_at(point(1.0, 0.0, 0.0)).unwrap();
        assert_eq!(vector(1.0, 0.0, 0.0), n);
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let s = Sphere::default();
        let n = s.normal_at(point(0.0, 1.0, 0.0)).unwrap();
        assert_eq!(vector(0.0, 1.0, 0.0), n);
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let s = Sphere::default();
        let n = s.normal_at(point(0.0, 0.0, 1.0)).unwrap();
        assert_eq!(vector(0.0, 0.0, 1.0), n);
    }

    #[test]
    fn normal_on_sphere_at_point_at_nonaxial_point() {
        let s = Sphere::default();
        let three_sqrt = 3.0f64.sqrt();
        let n = s
            .normal_at(point(three_sqrt / 3.0, three_sqrt / 3.0, three_sqrt / 3.0))
            .unwrap();
        assert_eq!(
            vector(three_sqrt / 3.0, three_sqrt / 3.0, three_sqrt / 3.0),
            n
        );
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = Sphere::default();
        let three_sqrt = 3.0f64.sqrt();
        let n = s
            .normal_at(point(three_sqrt / 3.0, three_sqrt / 3.0, three_sqrt / 3.0))
            .unwrap();
        assert_eq!(normalize(&n), n);
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let mut s = Sphere::default();
        s.set_transform(translation(0.0, 1.0, 0.0));
        let n = s
            .normal_at(point(0.0, 1.70711, -std::f64::consts::FRAC_1_SQRT_2))
            .unwrap(); // 0.70711
        assert_eq!(
            vector(
                0.0,
                std::f64::consts::FRAC_1_SQRT_2,
                -std::f64::consts::FRAC_1_SQRT_2
            ),
            n
        );
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut s = Sphere::default();
        s.set_transform(scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0));
        let two_sqrt = 2.0f64.sqrt();
        let n = s
            .normal_at(point(0.0, two_sqrt / 2.0, -two_sqrt / 2.0))
            .unwrap();
        assert_eq!(vector(0.0, 0.97014, -0.24254), n);
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::default();
        let default_m = Material::default();
        let m = s.material;
        assert_eq!(default_m, m);
    }

    #[test]
    fn sphere_may_have_assigned_material() {
        let mut s = Sphere::default();
        let m = Material {
            ambient: 1.0,
            ..Default::default()
        };
        s.set_material(m);
        assert_eq!(m, s.material);
    }
}
