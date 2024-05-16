use std::sync::{Arc, RwLock};
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;
use crate::common::execution::{Processor, WithOptions};
use crate::processors::internal::convolution_rgb::{ConvolutionRgbProcessor, ConvolutionRgbProcessorOptions};

pub struct MeanProcessorOptions {
    pub radius: usize,
    pub use_fast_approximation: bool,
}

impl Default for MeanProcessorOptions {
    fn default() -> MeanProcessorOptions {
        MeanProcessorOptions {
            radius: 3,
            use_fast_approximation: true,
        }
    }
}

pub struct MeanProcessor {
    options: MeanProcessorOptions,
}

impl MeanProcessor {
    pub fn new() -> Self {
        MeanProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<MeanProcessorOptions> for MeanProcessor {
    fn with_options(self, options: MeanProcessorOptions) -> Self {
        MeanProcessor { options }
    }
}

impl Processor for MeanProcessor {
    fn process(&self, fast_image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage {
        let radius = self.options.radius;

        let kernel = vec![1.0; (2 * radius + 1) * (2 * radius + 1)];
        let mut processor = ConvolutionRgbProcessor::new()
            .with_options(
                ConvolutionRgbProcessorOptions {
                    kernel,
                    kernel_width: 2 * radius + 1,
                    kernel_height: 2 * radius + 1,
                    kernel_divisor: (2 * radius + 1) as f32 * (2 * radius + 1) as f32,
                    kernel_offset: 0.0,
                    use_fast_approximation: self.options.use_fast_approximation,
                }
            );
        processor.process(fast_image, progress)
    }
}
