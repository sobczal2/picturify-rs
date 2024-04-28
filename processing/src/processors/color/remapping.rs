use crate::common::execution::{CpuOptions, ExecutionPlan, Processor};
use picturify_core::error::PicturifyResult;
use picturify_core::fast_image::apply_fn_to_pixels::{
    ApplyFnToPalettePixels,
};
use picturify_core::fast_image::FastImage;

use picturify_core::palette::{LinSrgba};

pub enum RemappingFunction {
    Linear {
        min: f32,
        max: f32,
    },
    Exponential {
        factor: f32,
    },
    Logarithmic {
        factor: f32,
    },
    Custom {
        map: fn(lin_srgb: LinSrgba) -> LinSrgba,
    },
}

impl RemappingFunction {
    fn apply_to_pixel(&self, pixel: LinSrgba) -> LinSrgba {
        match self {
            RemappingFunction::Linear { min, max } => {
                let r = pixel.red;
                let g = pixel.green;
                let b = pixel.blue;

                let new_r = (r - min) / (max - min);
                let new_g = (g - min) / (max - min);
                let new_b = (b - min) / (max - min);

                LinSrgba::new(new_r, new_g, new_b, pixel.alpha)
            }
            RemappingFunction::Exponential { factor } => {
                let r = pixel.red;
                let g = pixel.green;
                let b = pixel.blue;

                let new_r = r.powf(*factor);
                let new_g = g.powf(*factor);
                let new_b = b.powf(*factor);

                LinSrgba::new(new_r, new_g, new_b, pixel.alpha)
            }
            RemappingFunction::Logarithmic { factor } => {
                let r = pixel.red;
                let g = pixel.green;
                let b = pixel.blue;

                let new_r = (r + 1.).log(*factor);
                let new_g = (g + 1.).log(*factor);
                let new_b = (b + 1.).log(*factor);

                LinSrgba::new(new_r, new_g, new_b, pixel.alpha)
            }
            RemappingFunction::Custom { map } => map(pixel),
        }
    }
}

pub struct RemappingProcessorOptions {
    pub function: RemappingFunction,
}

impl Default for RemappingProcessorOptions {
    fn default() -> RemappingProcessorOptions {
        RemappingProcessorOptions {
            function: RemappingFunction::Linear { min: 0.0, max: 1.0 },
        }
    }
}

pub struct RemappingProcessor {
    execution_plan: ExecutionPlan,
    options: RemappingProcessorOptions,
}

impl Default for RemappingProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl RemappingProcessor {
    pub fn new() -> RemappingProcessor {
        RemappingProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options: Default::default(),
        }
    }

    pub fn with_options(options: RemappingProcessorOptions) -> RemappingProcessor {
        RemappingProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options,
        }
    }

    fn run_cpu(&self, mut fast_image: FastImage, cpu_options: CpuOptions) -> FastImage {
        cpu_options.build_thread_pool().install(|| {
            fast_image.par_apply_fn_to_lin_srgba(|pixel, _x, _y| {
                self.options.function.apply_to_pixel(pixel)
            });
        });

        fast_image
    }

    fn run_gpu(&self, _fast_image: FastImage) -> FastImage {
        unimplemented!()
    }
}

impl Processor for RemappingProcessor {
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
