use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;

pub trait Processor {
    fn process(&self, image: FastImage, progress: Progress) -> FastImage;
}
