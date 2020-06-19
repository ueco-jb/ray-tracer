#[derive(Debug)]
struct Projectile {
    position: tuple::Tuple,
    velocity: tuple::Tuple,
}

fn main() {
    let p: Projectile = Projectile {
        position: tuple::point(1, 1, 1),
        velocity: tuple::vector(2, 4, 6),
    };
    println!("Projectile: {:?}", p);
}
