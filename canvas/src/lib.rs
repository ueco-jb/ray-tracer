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

pub struct PPM {
    header: String,
    body: String,
}

pub fn scale_color(color: f64, max: f64) -> u32 {
    if color < 0.0_f64 || tuple::eq_with_eps(0.0_f64, color) {
        0
    } else if color > max || tuple::eq_with_eps(max, color) {
        max as u32
    } else {
        (color * max) as u32
    }
}

pub fn color_to_scaled_integers(c: &color::Color, max: f64) -> Box<[u32]> {
    let mut scaled = Box::new([0; 3]);
    scaled[0] = scale_color(c.get_red(), max);
    scaled[1] = scale_color(c.get_green(), max);
    scaled[2] = scale_color(c.get_blue(), max);
    scaled
}

pub fn canvas_to_ppm(c: &Canvas) -> PPM {
    let magic_number = "P3";
    let maximum_color_value: u32 = 255;
    let header = format!(
        "{}\n{} {}\n{}",
        magic_number, c.width, c.height, maximum_color_value
    );

    PPM {
        header,
        body: "".to_string(),
    }
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

    #[test]
    fn constructing_ppm_header() {
        let c = canvas(5, 3);
        let ppm = canvas_to_ppm(&c);
        assert_eq!("P3\n5 3\n255".to_string(), ppm.header);
    }

    // #[test]
    // fn constructing_ppm_body() {
    //     let c = canvas(5, 3);
    //     let c1 = color::color(1.5, 0, 0);
    //     let c2 = color::color(0, 0.5, 0);
    //     let c3 = color::color(-0.5, 0, 1);
    //     c.write_pixel(0, 0, c1);
    //     c.write_pixel(2, 1, c2);
    //     c.write_pixel(4, 2, c3);
    //     let ppm = canvas_to_ppm(&c);
    //     assert_eq!(format!("{}\n{}\n{}", "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", "0 0 0 0 0 0 128 0 0 0 0 0 0 0", "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"), ppm.body);
    // }
}
