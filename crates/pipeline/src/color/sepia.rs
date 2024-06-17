use picturify_core::core::fast_image::FastImage;
use picturify_core::error::pipeline::PipelinePicturifyResult;
use picturify_processing::common::execution::Processor;
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
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> PipelinePicturifyResult<FastImage> {
        let mut pipeline_progress = pipeline_progress.unwrap_or_default();

        pipeline_progress.new_individual(SEPIA_PROCESSOR_NAME.to_string());
        pipeline_progress.setup_combined(1);

        #[cfg(feature = "gpu")]
        let processor: Box<dyn Processor> = match self.options.use_gpu {
            true => Box::new(SepiaGpuProcessor::new(SepiaGpuProcessorOptions {})),
            false => Box::new(SepiaProcessor::new(SepiaProcessorOptions {
                use_fast_approximation: self.options.fast,
            })),
        };

        #[cfg(not(feature = "gpu"))]
        let processor: Box<dyn Processor> = Box::new(SepiaProcessor::new(SepiaProcessorOptions {
            use_fast_approximation: self.options.fast,
        }));

        let final_image =
            processor.process(image, pipeline_progress.get_current_individual_progress())?;
        pipeline_progress.increment_combined();
        
        Ok(final_image)
    }
}
