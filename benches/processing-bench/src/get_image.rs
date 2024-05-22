use crate::common::ImageResolution;
use criterion::{criterion_group, Criterion};

fn get_image_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_image_benchmark");
    let resolution_options = ImageResolution::get_resolutions();

    for resolution in resolution_options {
        group.bench_with_input(resolution.to_string(), &resolution, |b, &resolution| {
            b.iter(|| {
                resolution.get_image();
            });
        });
    }
}

criterion_group!(get_image_benchmark_group, get_image_benchmark);
