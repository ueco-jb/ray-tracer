use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use matches::assert_matches;
use ray_tracer::{
    canvas_to_ppm, intersect, lighting, normalize, point, vector, Canvas, Color, Computations,
    Intersection, Material, Matrix, Matrix4, PointLight, Ray, Sphere, World,
};
use std::{cell::RefCell, rc::Rc};

pub fn hit(c: &mut Criterion) {
    let s = Sphere::default();
    let ray_origin = point(0.0, 0.0, -5.0);
    let z_wall = 10f64;
    c.bench_function("Testing time to calculate hit", |b| {
        b.iter(|| {
            let position = point(50_f64, 50_f64, z_wall);
            let r = Ray {
                origin: ray_origin,
                direction: normalize(&(position - ray_origin)),
            };
            let mut xs = intersect(Rc::new(s), &r).unwrap();
            xs.hit();
        })
    });
}

pub fn computations(c: &mut Criterion) {
    let s = Sphere::default();
    for t in &[(-5.0, 4.0), (0.0, 1.0)] {
        c.bench_with_input(
            BenchmarkId::new(format!("hit with z: {z} and t: {t}", z = t.0, t = t.1), ""),
            t,
            |b, &t| {
                b.iter(|| {
                    let r = Ray {
                        origin: point(0.0, 0.0, t.0),
                        direction: vector(0.0, 0.0, 1.0),
                    };
                    let i = Intersection {
                        t: t.1,
                        object: RefCell::new(Rc::new(s)),
                    };
                    Computations::prepare_computation(i, r).unwrap();
                })
            },
        );
    }
}

#[allow(unused_variables)]
pub fn matrix(c: &mut Criterion) {
    let m = Matrix4([
        -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
    ]);
    let mut group = c.benchmark_group("Matrix operations");
    group.bench_function(BenchmarkId::new("inverse", ""), |b| {
        b.iter(|| {
            m.inverse().unwrap();
        })
    });
    let res = -4071.0;
    group.bench_function(BenchmarkId::new("determiant", ""), |b| {
        b.iter(|| {
            assert_matches!(m.determiant(), Ok(res));
        })
    });
    group.bench_function(BenchmarkId::new("transpose", ""), |b| {
        b.iter(|| {
            m.determiant().unwrap();
        })
    });
    group.finish();
}

pub fn canvas(c: &mut Criterion) {
    let mut canvas = Canvas::new(50, 30);
    let c1 = Color::new(1.5, 0.0, 0.0);
    let c2 = Color::new(0.0, 0.5, 0.0);
    let c3 = Color::new(-0.5, 0.0, 1.0);
    canvas.write_pixel(0, 0, c1).unwrap();
    canvas.write_pixel(2, 1, c2).unwrap();
    canvas.write_pixel(4, 2, c3).unwrap();
    c.bench_function("Transforming canvas to PPM format", |b| {
        b.iter(|| {
            canvas_to_ppm(&canvas);
        })
    });
}

pub fn reflections(c: &mut Criterion) {
    let m = Material::default();
    let position = point(0.0, 0.0, 0.0);
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = PointLight {
        position: point(0.0, 0.0, -10.0),
        intensity: Color::new(1.0, 1.0, 1.0),
    };
    c.bench_function("Reflections on light using Phong model", |b| {
        b.iter(|| {
            lighting(&m, light, position, eyev, normalv);
        })
    });
}

#[allow(unused_variables)]
pub fn world_intersections(c: &mut Criterion) {
    let mut w = World::default();
    let r = Ray {
        origin: point(0.0, 0.0, -5.0),
        direction: vector(0.0, 0.0, 1.0),
    };
    let result_color = Color::new(0.38066, 0.47583, 0.2855);
    c.bench_function("Color at object inside World when ray hits", |b| {
        b.iter(|| assert_matches!(w.color_at(&r), Ok(result_color)))
    });
}

criterion_group!(
    benchmark,
    hit,
    computations,
    matrix,
    canvas,
    reflections,
    world_intersections
);
criterion_main!(benchmark);
