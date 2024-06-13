use picturify_core::core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

use crate::common::execution::Processor;
use crate::helpers::kernels::{create_sobel_kernel_x, create_sobel_kernel_y};
use crate::processors::edge::gradient_based::{
    GradientBasedProcessor, GradientBasedProcessorOptions,
};

pub struct SobelProcessorOptions {
    pub use_fast_approximation: bool,
}

pub struct SobelProcessor {
    options: SobelProcessorOptions,
}

impl SobelProcessor {
    pub fn new(options: SobelProcessorOptions) -> Self {
        Self { options }
    }
}

impl Processor for SobelProcessor {
    fn process(&self, image: FastImage, progress: Progress) -> FastImage {
        let inner_processor_options = GradientBasedProcessorOptions {
            use_fast_approximation: self.options.use_fast_approximation,
            x_kernel: create_sobel_kernel_x(),
            y_kernel: create_sobel_kernel_y(),
        };
        let inner_processor = GradientBasedProcessor::new(inner_processor_options).unwrap();
        inner_processor.process(image, progress)
    }
}
