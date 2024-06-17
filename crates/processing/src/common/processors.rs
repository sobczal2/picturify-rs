use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::threading::progress::Progress;

pub trait CpuProcessor {
    fn process(&self, image: FastImage, progress: Progress)
        -> ProcessingPicturifyResult<FastImage>;
}

pub trait GpuProcessor {
    fn process(&self, image: FastImage) -> ProcessingPicturifyResult<FastImage>;
}