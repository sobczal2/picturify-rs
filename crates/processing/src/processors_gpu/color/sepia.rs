use crate::common::processors::GpuProcessor;
use crate::processors_gpu::common::fast_image::CFastImage;
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use std::ffi::c_int;

#[link(name = "picturify-processing-opencl", kind = "static")]
extern "C" {
    fn picturify_sepia(image: *mut CFastImage) -> c_int;
}

pub struct SepiaGpuProcessorOptions {}

#[allow(dead_code)]
pub struct SepiaGpuProcessor {
    options: SepiaGpuProcessorOptions,
}

impl SepiaGpuProcessor {
    pub fn new(options: SepiaGpuProcessorOptions) -> Self {
        Self { options }
    }
}

impl GpuProcessor for SepiaGpuProcessor {
    fn process(&self, image: FastImage) -> ProcessingPicturifyResult<FastImage> {
        let mut c_image = CFastImage::from_fast_image(image);
        let status = unsafe { picturify_sepia(&mut c_image) };

        if status != 0 {
            panic!("OpenCL error");
        }

        let image = unsafe { c_image.to_fast_image() };

        Ok(image)
    }
}
