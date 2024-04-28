use crate::common::execution::{CpuOptions, ExecutionPlan, Processor};
use picturify_core::error::PicturifyResult;
use picturify_core::fast_image::apply_fn_to_pixels::{
    ApplyFnToImagePixels, ApplyFnToPalettePixels,
};
use picturify_core::fast_image::FastImage;

pub struct NegativeProcessorOptions {
    pub use_fast_approximation: bool,
}

pub struct NegativeProcessor {
    execution_plan: ExecutionPlan,
    options: NegativeProcessorOptions,
}

impl Default for NegativeProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl NegativeProcessor {
    pub fn new() -> NegativeProcessor {
        NegativeProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options: NegativeProcessorOptions {
                use_fast_approximation: false,
            },
        }
    }

    pub fn with_options(options: NegativeProcessorOptions) -> NegativeProcessor {
        NegativeProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options,
        }
    }

    fn run_cpu(&self, mut fast_image: FastImage, cpu_options: CpuOptions) -> FastImage {
        cpu_options.build_thread_pool().install(|| {
            if self.options.use_fast_approximation {
                fast_image.par_apply_fn_to_image_pixel(|pixel, _x, _y| {
                    pixel.0[0] = 255 - pixel.0[0];
                    pixel.0[1] = 255 - pixel.0[1];
                    pixel.0[2] = 255 - pixel.0[2];
                });
            } else {
                fast_image.par_apply_fn_to_lin_srgba(|pixel, _x, _y| {
                    let mut pixel = pixel;
                    pixel.red = 1.0 - pixel.red;
                    pixel.green = 1.0 - pixel.green;
                    pixel.blue = 1.0 - pixel.blue;
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

impl Processor for NegativeProcessor {
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
