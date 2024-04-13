use picturify_core::fast_image::FastImage;

pub trait Pipeline {
    fn run(&self, fast_image: FastImage) -> FastImage;
}
