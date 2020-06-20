#![allow(dead_code)]

mod color;

#[derive(Debug)]
pub struct Canvas {
    width: u64,
    height: u64,
}

pub fn canvas(w: u64, h: u64) -> Canvas {
    Canvas {
        width: w,
        height: h,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let c = canvas(10, 20);
        let red = color::color(1, 0, 0);

    }
}
