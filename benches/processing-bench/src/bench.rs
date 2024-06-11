use crate::color::negative::negative_processor_benchmark_group;
use crate::get_image::get_image_benchmark_group;
use criterion::criterion_main;

pub mod color;
pub mod common;
pub mod get_image;

criterion_main!(
    get_image_benchmark_group,
    negative_processor_benchmark_group
);
