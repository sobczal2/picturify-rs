use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;
use std::sync::{Arc, RwLock};

pub trait Processor {
    fn process(&self, image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage;
}

pub trait WithOptions<TOptions> {
    fn with_options(self, options: TOptions) -> Self;
}
