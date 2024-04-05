use picturify_core::image::image::Image;

pub trait Processor {
    fn process(&self, image: Image) -> Image;
}

pub struct ThreadingOptions {
    num_threads: u32,
}