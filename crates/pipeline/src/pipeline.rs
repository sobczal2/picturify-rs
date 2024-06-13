use crate::common::pipeline_progress::PipelineProgress;
use picturify_core::core::fast_image::FastImage;

pub trait Pipeline {
    fn run(&self, image: FastImage, pipeline_progress: Option<PipelineProgress>) -> FastImage;
}
