use ray_tracer::{
    canvas_to_ppm, intersect, normalize, point, save, scaling, shearing, Canvas, Color, Ray, Shape,
    Sphere,
};
use std::boxed::Box;

const CANVAS_SIZE: usize = 100;

/// Sphere is at coordinates (0, 0, 0) by default with R=1.
/// Origin of ray is at (0, 0, -5) - so 5 units in front of center of sphere
/// Further the wall, bigger the shadow. Extrapolating data given before, wall at distance 10 on z
/// axis needs to be at least 6x6 to fill whole sphere's shadow. wall_size = 7 is a precautions.
/// Thanks to that, whole image scales with CANVAS_SIZE value - though larger images take
/// exponentially longer to render.

fn main() {
    let mut c = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);

    let mut s = Sphere::default();
    s.set_transform(shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * scaling(0.5, 1.0, 1.0));
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / CANVAS_SIZE as f64;
    let half = wall_size / 2.0;

    for y in 0..CANVAS_SIZE - 1 {
        // world coordinates differs from regular, because originally 0,0 is at left bottom corner
        let world_y = half - pixel_size * y as f64;
        for x in 0..CANVAS_SIZE - 1 {
            let world_x = -half + pixel_size * x as f64;
            let position = point(world_x, world_y, wall_z);
            let r = Ray {
                origin: ray_origin,
                direction: normalize(&(position - ray_origin)),
            };
            let xs = intersect(Box::new(s), &r).unwrap();
            if !(*xs).is_empty() {
                c.write_pixel(x, y, Color::new(0.85, 0.54, 0.48))
                    .expect("Out of canvas border");
            }
        }
    }

    let data = canvas_to_ppm(&c);
    save(&data.get(), "saved.ppm").unwrap();
}
