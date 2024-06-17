use crate::common::pipeline_progress::PipelineProgress;
use crate::pipeline::Pipeline;
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::pipeline::PipelinePicturifyResult;
use picturify_core::geometry::size::Size;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::geometry::scale::{
    ScaleProcessor, ScaleProcessorOptions, ScaleStrategy,
};

pub struct ScalePipelineOptions {
    pub size: Size,
    pub strategy: ScaleStrategy,
}

pub struct ScalePipeline {
    options: ScalePipelineOptions,
}

impl ScalePipeline {
    pub fn new(options: ScalePipelineOptions) -> Self {
        Self { options }
    }
}

const SCALE_PROCESSOR_NAME: &str = "Scale";

impl Pipeline for ScalePipeline {
    fn run(
        &self,
        image: FastImage,
        pipeline_progress: Option<PipelineProgress>,
    ) -> PipelinePicturifyResult<FastImage> {
        let mut pipeline_progress = pipeline_progress.unwrap_or_default();

        pipeline_progress.new_individual(SCALE_PROCESSOR_NAME.to_string());
        pipeline_progress.setup_combined(1);

        let processor = ScaleProcessor::new(ScaleProcessorOptions {
            size: self.options.size,
            strategy: self.options.strategy,
        });

        let final_image =
            processor.process(image, pipeline_progress.get_current_individual_progress())?;
        pipeline_progress.increment_combined();

        Ok(final_image)
    }
}
