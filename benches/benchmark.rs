use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use ray_tracer::{intersect, normalize, point, vector, Computations, Intersection, Ray, Sphere};
use std::{cell::RefCell, rc::Rc};

pub fn intersect_benchmark(c: &mut Criterion) {
    let s = Sphere::default();
    let ray_origin = point(0.0, 0.0, -5.0);
    let z_wall = 10f64;
    let mut group = c.benchmark_group("Hit calculation");
    group.bench_function(BenchmarkId::new("Testing time to calculate hit", ""), |b| {
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
    group.finish();
}

pub fn calculations_benchmark(c: &mut Criterion) {
    let s = Sphere::default();
    let mut group = c.benchmark_group("Calculations");
    for t in &[(-5.0, 4.0), (0.0, 1.0)] {
        group.bench_with_input(
            format!("hit with z: {z} and t: {t}", z = t.0, t = t.1),
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
    group.finish();
}

criterion_group!(benchmark, intersect_benchmark, calculations_benchmark);
criterion_main!(benchmark);
