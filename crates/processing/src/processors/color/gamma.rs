use picturify_core::fast_image::apply_fn_to_pixels::{ApplyFnToImagePixels, ApplyFnToPalettePixels};
use picturify_core::fast_image::FastImage;
use picturify_core::pixel::rgba::RgbaF32Pixel;
use picturify_core::threading::progress::Progress;
use crate::common::execution::{Processor, WithOptions};

pub struct GammaProcessorOptions {
    pub gamma: f32,
    pub use_fast_approximation: bool,
}

impl Default for GammaProcessorOptions {
    fn default() -> Self {
        GammaProcessorOptions {
            gamma: 1.0,
            use_fast_approximation: false,
        }
    }
}

pub struct GammaProcessor {
    options: GammaProcessorOptions,
}

impl GammaProcessor {
    pub fn new() -> Self {
        GammaProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<GammaProcessorOptions> for GammaProcessor {
    fn with_options(self, options: GammaProcessorOptions) -> Self {
        GammaProcessor { options }
    }
}

impl Processor for GammaProcessor {
    fn process(&self, image: FastImage, progress: Progress) -> FastImage {
        if self.options.use_fast_approximation {
            self.process_fast(image, progress)
        } else {
            self.process_slow(image, progress)
        }
    }
}

impl GammaProcessor {
    fn process_fast(&self, mut image: FastImage, progress: Progress) -> FastImage {
        image.par_apply_fn_to_image_pixel(
            |pixel, _coord| {
                let red = pixel.red_f32();
                let green = pixel.green_f32();
                let blue = pixel.blue_f32();
                
                let red = red.powf(self.options.gamma);
                let green = green.powf(self.options.gamma);
                let blue = blue.powf(self.options.gamma);
                
                pixel.set_red_clamped_f32(red);
                pixel.set_green_clamped_f32(green);
                pixel.set_blue_clamped_f32(blue);
            },
            Some(progress),
        );
        image
    }
    
    fn process_slow(&self, mut image: FastImage, progress: Progress) -> FastImage {
        image.par_apply_fn_to_lin_srgba(
            |pixel, _coord| {
                let red = pixel.red;
                let green = pixel.green;
                let blue = pixel.blue;
                
                let red = red.powf(self.options.gamma);
                let green = green.powf(self.options.gamma);
                let blue = blue.powf(self.options.gamma);
                
                pixel.red = red;
                pixel.green = green;
                pixel.blue = blue;
                pixel
            },
            Some(progress),
        );
        image
    }
    
}