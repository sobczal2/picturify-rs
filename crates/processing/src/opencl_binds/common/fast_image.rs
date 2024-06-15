use std::ffi::{c_float, c_int};
use picturify_core::core::fast_image::FastImage;

#[repr(C)]
pub struct CFastImage {
    pub data: *mut c_float,
    pub size: CSize,
}

impl CFastImage {
    pub fn new(data: *mut c_float, size: CSize) -> Self {
        Self { data, size }
    }
    
    pub fn from_fast_image(image: FastImage) -> Self {
        let (width, height): (i32, i32) = image.size().into();
        let size = CSize {
            width,
            height,
        };
        
        let u8_vec = image.to_rgba_vec();
        let mut f_vec = Vec::with_capacity(u8_vec.len());
        for i in 0..u8_vec.len() {
            f_vec.push(u8_vec[i] as f32 / 255.0);
        }
        
        let data = f_vec.as_mut_ptr();
        
        std::mem::forget(f_vec);
        
        Self::new(data, size)
    }
    
    pub unsafe fn to_fast_image(&self) -> FastImage {
        let mut u8_vec = vec![0; (self.size.width * self.size.height) as usize * 4];
        for i in 0..u8_vec.len() {
            u8_vec[i] = ((*self.data.offset(i as isize)) * 255.0) as u8;
        }
        
        println!("width: {}, height: {}", self.size.width, self.size.height);
        println!("u8_vec: {:?}", u8_vec.len());
        FastImage::from_rgba_vec((self.size.width as u32, self.size.height as u32).into(), u8_vec)
    }
}

#[repr(C)]
pub struct CSize {
    pub width: c_int,
    pub height: c_int,
}