use std::sync::{Arc, RwLock};
use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::{Progress};

pub trait Processor<TOptions> {
    fn new() -> Self;
    fn with_options(self, options: TOptions) -> Self;
    fn process(&self, fast_image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage;
}
