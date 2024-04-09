use palette::{encoding, LinSrgba, Srgba};
use palette::encoding::Srgb;
use palette::rgb::{Rgb, Rgba};

use picturify_core::error::PicturifyResult;
use picturify_core::image::apply_fn_to_pixels::{ApplyFnToImagePixels, ApplyFnToPalettePixels};
use picturify_core::image::fast_image::FastImage;

use crate::common::execution::{CpuOptions, ExecutionPlan, Processor};

pub struct SepiaProcessorOptions {
    pub use_fast_approximation: bool,
}

impl Default for SepiaProcessorOptions {
    fn default() -> SepiaProcessorOptions {
        SepiaProcessorOptions {
            use_fast_approximation: false,
        }
    }
}

pub struct SepiaProcessor {
    execution_plan: ExecutionPlan,
    options: SepiaProcessorOptions,
}

impl SepiaProcessor {
    pub fn new() -> SepiaProcessor {
        SepiaProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options: Default::default(),
        }
    }
    
    pub fn with_options(options: SepiaProcessorOptions) -> SepiaProcessor {
        SepiaProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options,
        }
    }
    

    fn run_cpu(&self, mut fast_image: FastImage, cpu_options: CpuOptions) -> FastImage {
        cpu_options.build_thread_pool().install(|| {
            if self.options.use_fast_approximation {
                fast_image.par_apply_fn_to_pixel(|pixel, _x, _y| {
                    let r = pixel.0[0];
                    let g = pixel.0[1];
                    let b = pixel.0[2];

                    let new_r = r * 0.393 + g * 0.769 + b * 0.189;
                    let new_g = r * 0.349 + g * 0.686 + b * 0.168;
                    let new_b = r * 0.272 + g * 0.534 + b * 0.131;

                    pixel.0[0] = new_r;
                    pixel.0[1] = new_g;
                    pixel.0[2] = new_b;
                });
            } else {
                fast_image.par_apply_fn_to_linsrgba(|mut pixel, _x, _y| {
                    let r = pixel.red;
                    let g = pixel.green;
                    let b = pixel.blue;

                    let new_r = r * 0.393 + g * 0.769 + b * 0.189;
                    let new_g = r * 0.349 + g * 0.686 + b * 0.168;
                    let new_b = r * 0.272 + g * 0.534 + b * 0.131;

                    pixel.red = new_r;
                    pixel.green = new_g;
                    pixel.blue = new_b;
                    
                    pixel
                });
            }
        });

        fast_image
    }

    fn run_gpu(&self, _fast_image: FastImage) -> FastImage {
        unimplemented!()
    }
}

impl Processor for SepiaProcessor {
    fn set_execution_plan(&mut self, execution_plan: ExecutionPlan) -> PicturifyResult<()> {
        self.execution_plan = execution_plan;
        Ok(())
    }

    fn process(&self, fast_image: FastImage) -> FastImage {
        match self.execution_plan {
            ExecutionPlan::Cpu(options) => self.run_cpu(fast_image, options),
            ExecutionPlan::Gpu => self.run_gpu(fast_image),
        }
    }
}
