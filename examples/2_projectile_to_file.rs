use ray_tracer::TupleT;
use ray_tracer::{
    canvas_to_ppm, eq_with_eps, normalize, point, save, vector, Canvas, Color, Tuple,
};

#[derive(Debug)]
struct Projectile {
    position: Tuple, // point
    velocity: Tuple, // vector
}

#[derive(Debug)]
struct Environment {
    gravity: Tuple, // vector
    wind: Tuple,    // vector
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

fn main() {
    let mut p: Projectile = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: normalize(&vector(1.0, 1.8, 0.0)) * 11.25,
    };
    let e: Environment = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };

    let mut c = Canvas::new(900, 550);

    println!("Starting conditions: {:?}", p);
    loop {
        p = tick(&e, &p);
        let xposition = p.position.get_x();
        let yposition = p.position.get_y();
        if eq_with_eps(0.0_f64, yposition) || yposition < 0.0_f64 {
            println!("Final position {:?}", p);
            break;
        } else {
            c.write_pixel(
                xposition as usize,
                c.get_height() - yposition as usize,
                Color::new(1.0, 1.0, 1.0),
            )
            .unwrap();
        }
    }
    let data = canvas_to_ppm(&c);
    save(&data.get(), "saved.ppm").unwrap();
}
