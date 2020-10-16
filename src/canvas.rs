use crate::color::Color;
use crate::utils::eq_with_eps;

const MAX_LINE_LENGTH: usize = 70;

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    canvas: std::vec::Vec<std::vec::Vec<Color>>,
}

#[derive(Debug)]
pub enum CanvasError {
    OutOfCanvasBorder,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Canvas {
        let vec = vec![vec![Color::new(0, 0, 0); h]; w];
        Canvas {
            width: w,
            height: h,
            canvas: vec,
        }
    }

    pub fn new_with_color(w: usize, h: usize, c: Color) -> Canvas {
        let vec = vec![vec![c; h]; w];
        Canvas {
            width: w,
            height: h,
            canvas: vec,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn pixel_at(&self, w: usize, h: usize) -> Result<Color, CanvasError> {
        if w > self.width - 1 || h > self.height - 1 {
            Err(CanvasError::OutOfCanvasBorder)
        } else {
            Ok(self.canvas[w][h])
        }
    }

    pub fn write_pixel(&mut self, w: usize, h: usize, c: Color) -> Result<(), CanvasError> {
        if w > self.width - 1 || h > self.height - 1 {
            Err(CanvasError::OutOfCanvasBorder)
        } else {
            self.canvas[w][h] = c;
            Ok(())
        }
    }
}

#[derive(Default)]
pub struct PPM {
    header: String,
    body: String,
}

impl PPM {
    pub fn get(&self) -> String {
        format!("{}\n{}", self.header, self.body)
    }
}

pub fn scale_color(color: f64, max: f64) -> u32 {
    if color < 0.0_f64 || eq_with_eps(0.0_f64, color) {
        0
    } else if color > 1.0_f64 || eq_with_eps(1.0_f64, color) {
        max as u32
    } else {
        (color * max).round() as u32
    }
}

fn color_to_scaled_integers(c: &Color, max: f64) -> Box<[u32]> {
    let mut scaled = Box::new([0; 3]);
    scaled[0] = scale_color(c.get_red(), max);
    scaled[1] = scale_color(c.get_green(), max);
    scaled[2] = scale_color(c.get_blue(), max);
    scaled
}

fn colors_to_scaled_vector(can: &Canvas, max: f64) -> String {
    let mut scaled_colors: String = "".to_string();
    for row in 0..can.get_height() {
        let mut temp_scaled_colors: String = "".to_string();
        for col in 0..can.get_width() {
            let pixel = can.pixel_at(col, row).unwrap();
            let scaled_pixel = color_to_scaled_integers(&pixel, max);
            temp_scaled_colors += &scaled_pixel[0].to_string();
            temp_scaled_colors += " ";
            temp_scaled_colors += &scaled_pixel[1].to_string();
            temp_scaled_colors += " ";
            temp_scaled_colors += &scaled_pixel[2].to_string();
            temp_scaled_colors += " ";
        }
        let len = temp_scaled_colors.len();
        temp_scaled_colors.truncate(len - 1);
        temp_scaled_colors += "\n";

        scaled_colors += &temp_scaled_colors
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if i != 0
                    && c == ' '
                    && (i % MAX_LINE_LENGTH == 0
                        || i % MAX_LINE_LENGTH == 67
                        || i % MAX_LINE_LENGTH == 68
                        || i % MAX_LINE_LENGTH == 69)
                {
                    Some('\n')
                } else {
                    None
                }
                .into_iter()
                .chain(std::iter::once(c))
            })
            .collect::<String>()
            .replace("\n ", "\n")[..];
    }
    scaled_colors
}

pub fn canvas_to_ppm(c: &Canvas) -> PPM {
    let magic_number = "P3";
    let maximum_color_value: u32 = 255;
    let header: String = format!(
        "{}\n{} {}\n{}",
        magic_number, c.width, c.height, maximum_color_value
    );
    let body: String = colors_to_scaled_vector(c, maximum_color_value as f64);
    PPM { header, body }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_pixel_on_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1, 0, 0);
        c.write_pixel(2, 3, red).unwrap();
        assert_eq!(red, c.pixel_at(2, 3).unwrap());
    }

    #[test]
    fn write_pixel_out_of_canvas_border() {
        let mut c = Canvas::new(1, 2);
        let red = Color::new(1, 0, 0);
        let write_result = c.write_pixel(20, 30, red);
        let get_result = c.pixel_at(20, 30);
        assert!(write_result.is_err());
        assert!(get_result.is_err());
    }

    #[test]
    fn constructing_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = canvas_to_ppm(&c);
        assert_eq!("P3\n5 3\n255".to_string(), ppm.header);
    }

    #[test]
    fn constructing_ppm_body() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0, 0);
        let c2 = Color::new(0, 0.5, 0);
        let c3 = Color::new(-0.5, 0, 1);
        c.write_pixel(0, 0, c1).unwrap();
        c.write_pixel(2, 1, c2).unwrap();
        c.write_pixel(4, 2, c3).unwrap();
        let ppm = canvas_to_ppm(&c);
        assert_eq!(
            format!(
                "{}\n{}\n{}\n",
                "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
            ),
            ppm.body
        );
    }

    #[test]
    fn constructing_ppm_body_splitting_lines() {
        let color = Color::new(1, 0.8, 0.6);
        let c = Canvas::new_with_color(10, 2, color);
        let ppm = canvas_to_ppm(&c);
        assert_eq!(
            format!(
                "{}\n{}\n{}\n{}\n",
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
            ),
            ppm.body
        );
    }
}
