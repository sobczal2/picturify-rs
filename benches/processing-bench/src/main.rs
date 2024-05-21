use criterion::criterion_main;
use crate::color::negative::negative_processor_benchmark_group;
use crate::get_image::get_image_benchmark_group;

pub mod common;
pub mod color;
pub mod get_image;

criterion_main!(
    get_image_benchmark_group,
    negative_processor_benchmark_group
);
