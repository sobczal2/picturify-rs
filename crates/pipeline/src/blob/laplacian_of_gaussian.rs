use crate::common::enlargement_crop_pipeline::{
    EnlargementCropPipeline, EnlargementCropPipelineOptions,
};
use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::pipeline::PipelinePicturifyResult;
use picturify_core::palette::Srgba;
use picturify_processing::processors::blob::laplacian_of_gaussian::{
    LaplacianOfGaussianProcessor, LaplacianOfGaussianProcessorOptions,
};
use picturify_processing::processors::geometry::crop::{CropBorder, CropProcessorOptions};
use picturify_processing::processors::geometry::enlargement::{
    EnlargementBorder, EnlargementProcessorOptions, EnlargementStrategy,
};

pub struct LaplacianOfGaussianPipelineOptions {
    pub radius: usize,
    pub sigma: f32,
    pub fast: bool,
}

pub struct LaplacianOfGaussianPipeline {
    options: LaplacianOfGaussianPipelineOptions,
}

impl LaplacianOfGaussianPipeline {
    pub fn new(options: LaplacianOfGaussianPipelineOptions) -> Self {
        Self { options }
    }
}

const LAPLACIAN_OF_GAUSSIAN_PROCESSOR_NAME: &str = "LaplacianOfGaussian";

impl Pipeline for LaplacianOfGaussianPipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> PipelinePicturifyResult<FastImage> {
        let processor = LaplacianOfGaussianProcessor::new(LaplacianOfGaussianProcessorOptions {
            radius: self.options.radius,
            sigma: self.options.sigma,
            use_fast_approximation: self.options.fast,
        });
        let (width, height) = image.size().into();
        let pipeline = EnlargementCropPipeline::new(EnlargementCropPipelineOptions {
            fast: self.options.fast,
            processor_name: LAPLACIAN_OF_GAUSSIAN_PROCESSOR_NAME.to_string(),
            processor: Box::new(processor),
            enlargement_processor_options: EnlargementProcessorOptions {
                strategy: EnlargementStrategy::Constant(Srgba::new(0.0, 0.0, 0.0, 1.0)),
                border: EnlargementBorder::from_all(self.options.radius),
            },
            crop_processor_options: CropProcessorOptions {
                crop_border: CropBorder::new(
                    width,
                    height,
                    self.options.radius,
                    self.options.radius,
                ),
            },
        });

        let image = pipeline.run(image, pipeline_progress)?;
        
        Ok(image)
    }
}
