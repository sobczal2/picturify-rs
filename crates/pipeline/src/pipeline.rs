use crate::common::pipeline_progress::PipelineProgress;
use picturify_core::fast_image::FastImage;
use std::sync::{Arc, RwLock};

pub trait Pipeline {
    fn run(
        &self,
        fast_image: FastImage,
        pipeline_progress: Option<Arc<RwLock<PipelineProgress>>>,
    ) -> FastImage;
}
