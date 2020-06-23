use ray_tracer::tuple;

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
        position: tuple::point(0, 1, 0),
        velocity: tuple::normalize(&tuple::vector(1, 1, 0)),
    };
    let e: Environment = Environment {
        gravity: tuple::vector(0, -0.1, 0),
        wind: tuple::vector(-0.01, 0, 0),
    };

    println!("Starting conditions: {:?}", p);
    let mut i: i32 = 0;
    loop {
        p = tick(&e, &p);
        println!("New tick #{}", i);
        i += 1;
        if tuple::eq_with_eps(0.0_f64, p.position.get_y()) || p.position.get_y() < 0.0_f64 {
            println!("Final position {:?}", p);
            break;
        }
        println!("Position {:?}", p);
    }
}
