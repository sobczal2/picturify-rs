use picturify_core::error::PicturifyResult;
use picturify_core::fast_image::apply_fn_to_pixels::{ApplyFnToImagePixels, ApplyFnToPalettePixels};
use picturify_core::fast_image::fast_image::FastImage;
use crate::common::execution::{CpuOptions, ExecutionPlan, Processor};

pub struct ThresholdProcessorOptions {
    pub red_threshold: u8,
    pub green_threshold: u8,
    pub blue_threshold: u8,
}

impl Default for ThresholdProcessorOptions {
    fn default() -> ThresholdProcessorOptions {
        ThresholdProcessorOptions {
            red_threshold: 128,
            green_threshold: 128,
            blue_threshold: 128,
        }
    }
}

pub struct ThresholdProcessor {
    execution_plan: ExecutionPlan,
    options: ThresholdProcessorOptions,
}

impl ThresholdProcessor {
    pub fn new() -> ThresholdProcessor {
        ThresholdProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options: Default::default(),
        }
    }

    pub fn with_options(options: ThresholdProcessorOptions) -> ThresholdProcessor {
        ThresholdProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options,
        }
    }

    fn run_cpu(&self, mut fast_image: FastImage, cpu_options: CpuOptions) -> FastImage {
        cpu_options.build_thread_pool().install(|| {
            fast_image.par_apply_fn_to_image_pixel(|pixel, _x, _y| {
                pixel.0[0] = if pixel.0[0] > self.options.red_threshold { pixel.0[0] } else { 0 };
                pixel.0[1] = if pixel.0[1] > self.options.green_threshold { pixel.0[1] } else { 0 };
                pixel.0[2] = if pixel.0[2] > self.options.blue_threshold { pixel.0[2] } else { 0 };
            });
        });

        fast_image
    }

    fn run_gpu(&self, _fast_image: FastImage) -> FastImage {
        unimplemented!()
    }
}

impl Processor for ThresholdProcessor {
    fn set_execution_plan(&mut self, execution_plan: ExecutionPlan) -> PicturifyResult<()> {
        self.execution_plan = execution_plan;
        Ok(())
    }

    fn process(&self, fast_image: FastImage) -> FastImage {
        match self.execution_plan {
            ExecutionPlan::Cpu(cpu_options) => self.run_cpu(fast_image, cpu_options),
            ExecutionPlan::Gpu => self.run_gpu(fast_image),
        }
    }
}

