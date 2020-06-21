#![allow(dead_code)]

mod color;

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    canvas: std::vec::Vec<std::vec::Vec<color::Color>>,
}

#[derive(Debug)]
pub enum CanvasError {
    OutOfCanvasBorder,
}

impl Canvas {
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn pixel_at(&self, w: usize, h: usize) -> Result<color::Color, CanvasError> {
        if w > self.width - 1 || h > self.height - 1 {
            Err(CanvasError::OutOfCanvasBorder)
        } else {
            Ok(self.canvas[w][h])
        }
    }

    pub fn write_pixel(&mut self, w: usize, h: usize, c: color::Color) -> Result<(), CanvasError> {
        if w > self.width - 1 || h > self.height - 1 {
            Err(CanvasError::OutOfCanvasBorder)
        } else {
            self.canvas[w][h] = c;
            Ok(())
        }
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

pub fn pixel_at(canvas: &Canvas, w: usize, h: usize) -> color::Color {
    canvas.pixel_at(w, h).unwrap()
}

pub fn write_pixel(canvas: &mut Canvas, w: usize, h: usize, c: color::Color) {
    canvas.write_pixel(w, h, c).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_pixel_on_canvas() {
        let mut c = canvas(10, 20);
        let red = color::color(1, 0, 0);
        c.write_pixel(2, 3, red).unwrap();
        assert_eq!(red, c.pixel_at(2, 3).unwrap());
    }

    #[test]
    fn write_pixel_out_of_canvas_border() {
        let mut c = canvas(1, 2);
        let red = color::color(1, 0, 0);
        let write_result = c.write_pixel(20, 30, red);
        let get_result = c.pixel_at(20, 30);
        assert!(write_result.is_err());
        assert!(get_result.is_err());
    }
}
