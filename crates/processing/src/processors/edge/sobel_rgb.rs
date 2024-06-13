use crate::common::execution::Processor;
use crate::helpers::kernels::{create_sobel_kernel_x, create_sobel_kernel_y};
use crate::processors::edge::gradient_based_rgb::{
    GradientBasedRgbProcessor, GradientBasedRgbProcessorOptions,
};
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

pub struct SobelRgbProcessorOptions {
    pub use_fast_approximation: bool,
}

impl Default for SobelRgbProcessorOptions {
    fn default() -> Self {
        SobelRgbProcessorOptions {
            use_fast_approximation: false,
        }
    }
}

pub struct SobelRgbProcessor {
    options: SobelRgbProcessorOptions,
}

impl SobelRgbProcessor {
    pub fn new(options: SobelRgbProcessorOptions) -> Self {
        Self { options }
    }
}

impl Processor for SobelRgbProcessor {
    fn process(&self, image: FastImage, progress: Progress) -> FastImage {
        let inner_processor_options = GradientBasedRgbProcessorOptions {
            use_fast_approximation: self.options.use_fast_approximation,
            x_kernel: create_sobel_kernel_x(),
            y_kernel: create_sobel_kernel_y(),
        };
        let inner_processor = GradientBasedRgbProcessor::new(inner_processor_options).unwrap();
        inner_processor.process(image, progress)
    }
}
