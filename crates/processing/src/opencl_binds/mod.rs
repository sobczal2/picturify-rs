use crate::opencl_binds::common::fast_image::CFastImage;

pub mod common;

#[link(name = "picturify-processing-opencl", kind = "static")]
extern {
    pub fn half_image(fast_image: *mut CFastImage);
}