use crate::common::execution::{CpuOptions, ExecutionPlan, Processor};
use picturify_core::error::PicturifyResult;
use picturify_core::fast_image::apply_fn_to_pixels::ApplyFnToPalettePixels;
use picturify_core::fast_image::FastImage;
use picturify_core::palette::Srgba;

pub enum EnlargementStrategy {
    Constant(Srgba),
    Mirror,
}

pub struct EnlargementProcessor {
    execution_plan: ExecutionPlan,
    options: EnlargementProcessorOptions,
}

pub struct EnlargementProcessorOptions {
    pub border: usize,
    pub strategy: EnlargementStrategy,
}

impl EnlargementProcessor {
    pub fn new(radius: usize) -> EnlargementProcessor {
        EnlargementProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options: EnlargementProcessorOptions {
                border: radius,
                strategy: EnlargementStrategy::Mirror,
            },
        }
    }

    pub fn with_options(
        edge_enlargement_processor_options: EnlargementProcessorOptions,
    ) -> EnlargementProcessor {
        EnlargementProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options: edge_enlargement_processor_options,
        }
    }

    fn run_cpu(&self, fast_image: FastImage, cpu_options: CpuOptions) -> FastImage {
        let new_width = fast_image.get_width() + self.options.border * 2;
        let new_height = fast_image.get_height() + self.options.border * 2;

        let mut new_image = FastImage::empty(new_width, new_height);

        // let width = fast_image.get_width();
        // let height = fast_image.get_height();

        cpu_options.build_thread_pool().install(|| {
            match self.options.strategy {
                EnlargementStrategy::Constant(pixel) => {
                    new_image.par_apply_fn_to_pixel(|_, x, y| {
                        if x < self.options.border
                            || x >= new_width - self.options.border
                            || y < self.options.border
                            || y >= new_height - self.options.border
                        {
                            pixel
                        } else {
                            fast_image
                                .get_srgba_pixel(x - self.options.border, y - self.options.border)
                        }
                    });
                }
                EnlargementStrategy::Mirror => {
                    // new_image.par_apply_fn_to_srgba(|new_pixel, x, y| {
                    //     if x < self.options.border
                    //         || x >= new_width - self.options.border
                    //         || y < self.options.border
                    //         || y >= new_height - self.options.border
                    //     {
                    //         let x = if x < self.options.border {
                    //             self.options.border - x - 1
                    //         } else if x >= width + self.options.border {
                    //             2 * width + self.options.border - x - 1
                    //         } else {
                    //             x - self.options.border
                    //         };
                    //
                    //         let y = if y < self.options.border {
                    //             self.options.border - y - 1
                    //         } else if y >= height + self.options.border {
                    //             2 * height + self.options.border - y - 1
                    //         } else {
                    //             y - self.options.border
                    //         };
                    //
                    //         fast_image.get_srgba_pixel(
                    //             if x > new_width / 2 {
                    //                 (new_width - x - 1) % width
                    //             } else {
                    //                 x % width
                    //             },
                    //             if y > new_height / 2 {
                    //                 (new_height - y - 1) % height
                    //             } else {
                    //                 y % height
                    //             },
                    //         )
                    //     } else {
                    //         fast_image
                    //             .get_srgba_pixel(x - self.options.border, y - self.options.border)
                    //     }
                    // });
                    unimplemented!("Mirror strategy is not implemented")
                }
            }
        });

        new_image
    }

    fn run_gpu(&self, _fast_image: FastImage) -> FastImage {
        unimplemented!()
    }
}

impl Processor for EnlargementProcessor {
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
