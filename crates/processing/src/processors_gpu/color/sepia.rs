use crate::common::execution::Processor;
use crate::processors_gpu::common::fast_image::CFastImage;
use log::warn;
use picturify_core::core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;
use std::ffi::c_int;

#[link(name = "picturify-processing-opencl", kind = "static")]
extern "C" {
    fn picturify_sepia(image: *mut CFastImage) -> c_int;
}

pub struct SepiaGpuProcessorOptions {}

pub struct SepiaGpuProcessor {
    options: SepiaGpuProcessorOptions,
}

impl SepiaGpuProcessor {
    pub fn new(options: SepiaGpuProcessorOptions) -> Self {
        Self { options }
    }
}

impl Processor for SepiaGpuProcessor {
    fn process(&self, image: FastImage, _progress: Progress) -> FastImage {
        warn!("Progress tracking is not supported for GPU processors");
        let mut c_image = CFastImage::from_fast_image(image);
        let status = unsafe { picturify_sepia(&mut c_image) };

        if status != 0 {
            panic!("OpenCL error");
        }

        unsafe { c_image.to_fast_image() }
    }
}
