extern crate ray_tracer as rt;

use color::Color;
use intersections::intersect;
use light::PointLight;
use material::lighting;
use ray::Ray;
use rt::*;
use shape::Shape;
use sphere::Sphere;
use tuple::{normalize, point};

const CANVAS_SIZE: usize = 100;

fn main() {
    let mut c: canvas::Canvas = canvas::Canvas::new(CANVAS_SIZE, CANVAS_SIZE);

    // light configuration - white light behind, above and to the left of the eye
    let light = PointLight {
        position: point(-10.0, 10.0, -10.0),
        intensity: Color::new(1.0, 1.0, 1.0),
    };

    let mut s: Sphere = Default::default();
    s.set_color(Color::new(1.0, 0.2, 1.0));

    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / CANVAS_SIZE as f64;
    let half = wall_size / 2.0;

    for y in 0..CANVAS_SIZE - 1 {
        // world coordinates differs from regular, because originally 0,0 is at left bottom corner
        let world_y = half - pixel_size * y as f64;
        for x in 0..CANVAS_SIZE - 1 {
            let world_x = -half + pixel_size * x as f64;
            let position = point(world_x, world_y, wall_z);
            let r = Ray {
                origin: ray_origin,
                direction: normalize(&(position - ray_origin)),
            };
            let mut xs = intersect(s, &r).unwrap();
            let hit = xs.hit();
            if let Some(hit) = hit {
                let point = r.position(hit.t);
                let normal = hit.object.normal_at(point).unwrap();
                let eye = -r.direction;
                let col = lighting(hit.object.clone().get_material(), light, point, eye, normal);
                c.write_pixel(x, y, col).expect("Out of canvas border");
            }
        }
    }

    let data: canvas::PPM = canvas::canvas_to_ppm(&c);
    serialize::save(&data.get(), "saved.ppm").unwrap();
}
