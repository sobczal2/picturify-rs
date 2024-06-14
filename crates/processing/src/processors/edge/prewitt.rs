use crate::common::execution::Processor;
use crate::helpers::kernels::{create_prewitt_kernel_x, create_prewitt_kernel_y};
use crate::processors::edge::gradient_based::{
    GradientBasedProcessor, GradientBasedProcessorOptions,
};
use picturify_core::core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

pub struct PrewittProcessorOptions {
    pub use_fast_approximation: bool,
}

pub struct PrewittProcessor {
    options: PrewittProcessorOptions,
}

impl PrewittProcessor {
    pub fn new(options: PrewittProcessorOptions) -> Self {
        Self { options }
    }
}

impl Processor for PrewittProcessor {
    fn process(&self, image: FastImage, progress: Progress) -> FastImage {
        let inner_processor_options = GradientBasedProcessorOptions {
            use_fast_approximation: self.options.use_fast_approximation,
            x_kernel: create_prewitt_kernel_x(),
            y_kernel: create_prewitt_kernel_y(),
        };
        let inner_processor = GradientBasedProcessor::new(inner_processor_options).unwrap();
        inner_processor.process(image, progress)
    }
}
