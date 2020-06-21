#![allow(dead_code)]

mod color;

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    canvas: std::vec::Vec<std::vec::Vec<color::Color>>,
}

impl Canvas {
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn pixel_at(&self, w: usize, h: usize) -> color::Color {
        self.canvas[w][h]
    }

    pub fn write_pixel(&mut self, w: usize, h: usize, c: color::Color) {
        self.canvas[w][h] = c;
    }
}

pub fn canvas(w: usize, h: usize) -> Canvas {
    let vec = vec![vec![color::color(0, 0, 0); w]; h];
    Canvas {
        width: w,
        height: h,
        canvas: vec,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let mut c = canvas(10, 20);
        let red = color::color(1, 0, 0);
        c.write_pixel(2, 3, red);
        assert_eq!(red, c.pixel_at(2, 3));
    }
}
