use picturify_core::core::fast_image::FastImage;
use picturify_core::error::pipeline::PipelinePicturifyResult;
use picturify_core::log_warn;
use picturify_processing::common::processors::{CpuProcessor, GpuProcessor};
use picturify_processing::processors::color::sepia::{SepiaProcessor, SepiaProcessorOptions};
#[cfg(feature = "gpu")]
use picturify_processing::processors_gpu::color::sepia::{
    SepiaGpuProcessor, SepiaGpuProcessorOptions,
};

use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;

pub struct SepiaPipelineOptions {
    pub fast: bool,
    #[cfg(feature = "gpu")]
    pub use_gpu: bool,
}

pub struct SepiaPipeline {
    options: SepiaPipelineOptions,
}

impl SepiaPipeline {
    pub fn new(options: SepiaPipelineOptions) -> Self {
        Self { options }
    }
}

const SEPIA_PROCESSOR_NAME: &str = "Sepia";

impl Pipeline for SepiaPipeline {
    fn run(
        &self,
        image: FastImage,
        pipeline_progress: Option<PipelineProgress>,
    ) -> PipelinePicturifyResult<FastImage> {
        let mut pipeline_progress = pipeline_progress.unwrap_or_default();

        pipeline_progress.new_individual(SEPIA_PROCESSOR_NAME.to_string());
        pipeline_progress.setup_combined(1);

        let final_image = self.process_image(image, pipeline_progress)?;

        Ok(final_image)
    }
}

impl SepiaPipeline {
    pub fn process_image_cpu(&self, image: FastImage, pipeline_progress: PipelineProgress) -> PipelinePicturifyResult<FastImage> {
        let processor = SepiaProcessor::new(SepiaProcessorOptions {
            use_fast_approximation: self.options.fast,
        });

        let final_image = processor.process(image, pipeline_progress.get_current_individual_progress())?;
        pipeline_progress.increment_combined();

        Ok(final_image)
    }
    
    #[cfg(feature = "gpu")]
    pub fn process_image_gpu(&self, image: FastImage, pipeline_progress: PipelineProgress) -> PipelinePicturifyResult<FastImage> {
        let processor = SepiaGpuProcessor::new(SepiaGpuProcessorOptions {});

        if self.options.fast {
            log_warn!("Fast approximation is not available for GPU processing");
        };

        log_warn!("GPU processing does not support progress reporting");

        let final_image = processor.process(image)?;
        pipeline_progress.increment_combined();

        Ok(final_image)
    }

    #[cfg(not(feature = "gpu"))]
    pub fn process_image(&self, image: FastImage, pipeline_progress: PipelineProgress) -> PipelinePicturifyResult<FastImage> {
        self.process_image_cpu(image, pipeline_progress)
    }

    #[cfg(feature = "gpu")]
    pub fn process_image(&self, image: FastImage, pipeline_progress: PipelineProgress) -> PipelinePicturifyResult<FastImage> {
        if self.options.use_gpu {
            self.process_image_gpu(image, pipeline_progress)
        } else {
            self.process_image_cpu(image, pipeline_progress)
        }
    }
}
