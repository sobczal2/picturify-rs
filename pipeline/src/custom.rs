use crate::pipeline::Pipeline;
use picturify_core::fast_image::FastImage;

pub struct CustomPipeline {
    function: fn(fast_image: FastImage) -> FastImage,
}

impl CustomPipeline {
    pub fn new(function: fn(fast_image: FastImage) -> FastImage) -> CustomPipeline {
        CustomPipeline { function }
    }
}

impl Pipeline for CustomPipeline {
    fn run(&self, fast_image: FastImage) -> FastImage {
        (self.function)(fast_image)
    }
}
