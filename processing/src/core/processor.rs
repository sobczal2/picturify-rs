use image::DynamicImage;

pub trait Processor {
    fn process(&self, image: DynamicImage) -> DynamicImage;
}

pub struct ThreadingOptions {
    num_threads: u32,
}