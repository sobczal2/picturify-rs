use crate::common::processors::CpuProcessor;
use crate::common::kernels::convolution::ConvolutionKernel;
use crate::processors::internal::convolution_rgb::{
    ConvolutionRgbProcessor, ConvolutionRgbProcessorOptions,
};
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::threading::progress::Progress;

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

impl CpuProcessor for LaplacianOfGaussianProcessor {
    fn process(
        &self,
        image: FastImage,
        progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let radius = self.options.radius;
        let sigma = self.options.sigma;

        let kernel = ConvolutionKernel::new_laplacian_of_gaussian(radius, sigma);

        let processor = ConvolutionRgbProcessor::new(ConvolutionRgbProcessorOptions {
            kernel,
            use_fast_approximation: self.options.use_fast_approximation,
        });

        processor.process(image, progress)
    }
}
