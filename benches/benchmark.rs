use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use ray_tracer::{intersect, normalize, point, Ray, Sphere};
use std::rc::Rc;

pub fn intersect_benchmark(c: &mut Criterion) {
    let s = Sphere::default();
    let ray_origin = point(0.0, 0.0, -5.0);
    let z_wall = 10f64;
    let mut group = c.benchmark_group("Hit calculation");
        group.bench_function(
            BenchmarkId::new("Testing time to calculate hit", ""),
            |b| {
                b.iter(|| {
                    let position = point(50_f64, 50_f64, z_wall);
                    let r = Ray {
                        origin: ray_origin,
                        direction: normalize(&(position - ray_origin)),
                    };
                    let mut xs = intersect(Rc::new(s), &r).unwrap();
                    xs.hit();
                })
            },
        );
    group.finish();
}



criterion_group!(benchmark, intersect_benchmark);
criterion_main!(benchmark);
