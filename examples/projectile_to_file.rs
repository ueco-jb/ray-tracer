extern crate ray_tracer as rt;

use crate::rt::tuple::TupleT;
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

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

fn main() {
    let mut p: Projectile = Projectile {
        position: tuple::point(0.0, 1.0, 0.0),
        velocity: tuple::normalize(&tuple::vector(1.0, 1.8, 0.0)) * 11.25,
    };
    let e: Environment = Environment {
        gravity: tuple::vector(0.0, -0.1, 0.0),
        wind: tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut c: canvas::Canvas =
        canvas::Canvas::new_with_color(900, 550, color::Color::new(0, 0, 0));

    println!("Starting conditions: {:?}", p);
    loop {
        p = tick(&e, &p);
        let xposition = p.position.get_x();
        let yposition = p.position.get_y();
        if utils::eq_with_eps(0.0_f64, yposition) || yposition < 0.0_f64 {
            println!("Final position {:?}", p);
            break;
        } else {
            c.write_pixel(
                xposition as usize,
                c.get_height() - yposition as usize,
                color::Color::new(1, 1, 1),
            )
            .unwrap();
        }
    }
    let data: canvas::PPM = canvas::canvas_to_ppm(&c);
    serialize::save(&data.get(), "saved.ppm").unwrap();
}
