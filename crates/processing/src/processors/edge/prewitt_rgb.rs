use crate::common::execution::Processor;
use crate::helpers::kernels::{create_prewitt_kernel_x, create_prewitt_kernel_y};
use crate::processors::edge::gradient_based_rgb::{
    GradientBasedRgbProcessor, GradientBasedRgbProcessorOptions,
};
use picturify_core::core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

pub struct PrewittRgbProcessorOptions {
    pub use_fast_approximation: bool,
}

pub struct PrewittRgbProcessor {
    options: PrewittRgbProcessorOptions,
}

impl PrewittRgbProcessor {
    pub fn new(options: PrewittRgbProcessorOptions) -> Self {
        Self { options }
    }
}

impl Processor for PrewittRgbProcessor {
    fn process(&self, image: FastImage, progress: Progress) -> FastImage {
        let inner_processor_options = GradientBasedRgbProcessorOptions {
            use_fast_approximation: self.options.use_fast_approximation,
            x_kernel: create_prewitt_kernel_x(),
            y_kernel: create_prewitt_kernel_y(),
        };
        let inner_processor = GradientBasedRgbProcessor::new(inner_processor_options).unwrap();
        inner_processor.process(image, progress)
    }
}