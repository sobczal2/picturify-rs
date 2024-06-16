use picturify_core::core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;
use crate::common::execution::Processor;
use crate::common::kernels::convolution::ConvolutionKernel;
use crate::processors::internal::convolution_rgb::{ConvolutionRgbProcessor, ConvolutionRgbProcessorOptions};

pub struct LaplacianOfGaussianProcessorOptions {
    pub radius: usize,
    pub sigma: f32,
    pub use_fast_approximation: bool,
}

pub struct LaplacianOfGaussianProcessor {
    options: LaplacianOfGaussianProcessorOptions,
}

impl LaplacianOfGaussianProcessor {
    pub fn new(options: LaplacianOfGaussianProcessorOptions) -> Self {
        Self { options }
    }
}

impl Processor for LaplacianOfGaussianProcessor {
    fn process(&self, image: FastImage, progress: Progress) -> FastImage {
        let radius = self.options.radius;
        let sigma = self.options.sigma;

        let processor = ConvolutionRgbProcessor::new(ConvolutionRgbProcessorOptions {
            kernel: ConvolutionKernel::new_laplacian_of_gaussian(radius, sigma),
            use_fast_approximation: self.options.use_fast_approximation,
        });

        processor.process(image, progress)
    }
}