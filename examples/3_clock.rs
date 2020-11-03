extern crate ray_tracer as rt;

use rt::tuple::TupleT;
use rt::*;
use transformations::rotation_z;
use tuple::point;
use utils::PI;

fn main() {
    let mut c: canvas::Canvas = canvas::Canvas::new(550, 550);

    let start = point(0.0, 200.0, 0.0);
    for n in 1..13 {
        let p = start * rotation_z((n as f64 * PI) / 6.0);
        let x = p.get_x() + c.get_width() as f64 / 2.0;
        let y = p.get_y() + c.get_height() as f64 / 2.0;
        c.write_pixel(x as usize, y as usize, color::Color::new(1.0, 1.0, 1.0))
            .expect("Out of canvas border");
    }

    let data: canvas::PPM = canvas::canvas_to_ppm(&c);
    serialize::save(&data.get(), "saved.ppm").unwrap();
}
