use picturify_core::error::PicturifyResult;
use picturify_core::fast_image::apply_fn_to_pixels::ApplyFnToImagePixels;
use picturify_core::fast_image::fast_image::FastImage;

use crate::common::execution::{CpuOptions, ExecutionPlan, Processor};

pub struct CropProcessorOptions {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub struct CropProcessor {
    execution_plan: ExecutionPlan,
    options: CropProcessorOptions,
}

impl CropProcessor {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> CropProcessor {
        CropProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options: CropProcessorOptions {
                x,
                y,
                width,
                height,
            },
        }
    }

    fn run_cpu(&self, fast_image: FastImage, cpu_options: CpuOptions) -> FastImage {
        let mut new_image = FastImage::empty(self.options.width, self.options.height);

        cpu_options.build_thread_pool().install(|| {
            new_image.par_apply_fn_to_image_pixel(|pixel, x, y| {
                let new_pixel = fast_image.get_image_pixel(x + self.options.x, y + self.options.y);
                *pixel = new_pixel;
            });
        });
        
        new_image
    }

    fn run_gpu(&self, _fast_image: FastImage) -> FastImage {
        unimplemented!()
    }
}

impl Processor for CropProcessor {
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