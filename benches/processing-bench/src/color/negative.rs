use std::fmt::Display;

use criterion::{BenchmarkId, Criterion, criterion_group};

use picturify_core::threading::progress::Progress;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::color::negative::{
    NegativeProcessor, NegativeProcessorOptions,
};

use crate::common::ImageResolution;

#[derive(Clone, Copy, Debug)]
struct NegativeProcessorBenchmarkOptions {
    resolution: ImageResolution,
    use_fast_approximation: bool,
}

impl Display for NegativeProcessorBenchmarkOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.use_fast_approximation {
            true => write!(f, "{}_fast_approximation", self.resolution),
            false => write!(f, "{}_full", self.resolution),
        }
    }
}

fn negative_processor_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("negative_processor_benchmark");
    let resolution_options = ImageResolution::get_resolutions();
    let use_fast_approximation_options = [true, false];
    let negative_processor_benchmark_options = resolution_options.iter().flat_map(|resolution| {
        use_fast_approximation_options
            .iter()
            .map(
                move |&use_fast_approximation| NegativeProcessorBenchmarkOptions {
                    resolution: *resolution,
                    use_fast_approximation,
                },
            )
    });
    for option in negative_processor_benchmark_options {
        group.bench_with_input(
            BenchmarkId::from_parameter(option),
            &option,
            |b, &options| {
                b.iter(|| {
                    let image = options.resolution.get_image();
                    let processor =
                        NegativeProcessor::new().with_options(NegativeProcessorOptions {
                            use_fast_approximation: options.use_fast_approximation,
                        });
                    processor.process(image, Progress::new());
                });
            },
        );
    }
}

criterion_group!(
    negative_processor_benchmark_group,
    negative_processor_benchmark
);
