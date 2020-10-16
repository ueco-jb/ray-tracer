extern crate ray_tracer as rt;

use rt::*;

#[derive(Debug)]
struct Projectile {
    position: tuple::Tuple, // point
    velocity: tuple::Tuple, // vector
}

#[derive(Debug)]
struct Environment {
    gravity: tuple::Tuple, // vector
    wind: tuple::Tuple,    // vector
}

fn main() {
    let mut c: canvas::Canvas =
        canvas::Canvas::new_with_color(900, 550, color::Color::new(0, 0, 0));

    // c.write_pixel(
    //     xposition as usize,
    //     c.get_height() - yposition as usize,
    //     color::Color::new(1, 1, 1),
    // )
    // .unwrap();

    let data: canvas::PPM = canvas::canvas_to_ppm(&c);
    serialize::save(&data.get(), "saved.ppm").unwrap();
}
