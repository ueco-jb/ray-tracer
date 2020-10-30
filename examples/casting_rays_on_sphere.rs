extern crate ray_tracer as rt;

use intersections::intersect;
use ray::Ray;
use rt::shape::Shape;
use rt::*;
use sphere::Sphere;
use transformations::{scaling, translation};
use tuple::{point, vector};

fn main() {
    let mut c: canvas::Canvas = canvas::Canvas::new(550, 550);

    let trans = translation(250.0, 250.0, 0.0);
    let scal = scaling(75.0, 75.0, 75.0);
    let mut s: Sphere = Default::default();
    s.set_transform(trans * scal);

    for r1 in 100..400 {
        for r2 in 100..400 {
            let r = Ray {
                origin: point(r1 as f64, r2 as f64, -300.0),
                direction: vector(0.0, 0.0, 1.0),
            };
            let xs = intersect(&s, &r).unwrap();
            if !xs.0.is_empty() {
                c.write_pixel(
                    r1 as usize,
                    r2 as usize,
                    color::Color::new(0.85, 0.81, 0.72),
                )
                .expect("Out of canvas border");
            }
        }
    }

    let data: canvas::PPM = canvas::canvas_to_ppm(&c);
    serialize::save(&data.get(), "saved.ppm").unwrap();
}
