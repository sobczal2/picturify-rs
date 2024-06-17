use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::threading::progress::Progress;

pub trait Processor {
    fn process(&self, image: FastImage, progress: Progress) -> ProcessingPicturifyResult<FastImage>;
}
