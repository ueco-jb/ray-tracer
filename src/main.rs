pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: u32,
}

impl Tuple {
    fn is_vector(&self) -> Result<bool, &'static str> {
        match self.w {
            0 => Ok(true),
            1 => Ok(false),
            _ => Err("invalid w value"),
        }
    }

    fn is_point(&self) -> Result<bool, &'static str> {
        match self.w {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err("invalid w value"),
        }
    }

    fn get_x(&self) -> &f64 {
        &self.x
    }
    fn get_y(&self) -> &f64 {
        &self.y
    }
    fn get_z(&self) -> &f64 {
        &self.z
    }
}

fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x: x,
        y: y,
        z: z,
        w: 1
    }
}

fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x: x,
        y: y,
        z: z,
        w: 0
    }
}

fn main() {
    let t: Tuple = point(1.0, 1.0, 1.0);
    println!("{}", t.is_vector().unwrap());
}
