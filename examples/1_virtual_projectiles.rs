use ray_tracer::TupleT;
use ray_tracer::{eq_with_eps, normalize, point, vector, Tuple};

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
        velocity: normalize(&vector(1.0, 1.0, 0.0)),
    };
    let e: Environment = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };

    println!("Starting conditions: {:?}", p);
    let mut i: i32 = 0;
    loop {
        p = tick(&e, &p);
        println!("New tick #{}", i);
        i += 1;
        if eq_with_eps(0.0_f64, p.position.get_y()) || p.position.get_y() < 0.0_f64 {
            println!("Final position {:?}", p);
            break;
        }
        println!("Position {:?}", p);
    }
}
