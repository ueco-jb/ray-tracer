mod tuple;

use tuple::*;

fn main() {
    let t: Tuple = point(1.0, 1.0, 1.0);
    let w: Tuple = point(1.0, 1.0, 1.0);
    println!("{}", t.is_vector().unwrap());
}
