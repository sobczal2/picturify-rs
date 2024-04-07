use crate::common::channel::ChannelSelector;
use crate::common::execution::{CpuOptions, ExecutionPlan};
use crate::common::process::Processor;
use picturify_core::error::processing::ProcessingError;
use picturify_core::error::PicturifyResult;
use picturify_core::image::fast_image::FastImage;
use picturify_core::image::pixel::RgbaPixel;
use picturify_core::image::virtual_image::{VirtualImage, VirtualRgbaImage};
use rayon::ThreadPoolBuilder;

pub struct RgbaPixelManipulationProcessor {
    execution_plan: ExecutionPlan,
    channel_selector: ChannelSelector,
    red_function: fn(rgb_pixel: RgbaPixel, x: usize, y: usize) -> u8,
    green_function: fn(rgb_pixel: RgbaPixel, x: usize, y: usize) -> u8,
    blue_function: fn(rgb_pixel: RgbaPixel, x: usize, y: usize) -> u8,
    alpha_function: fn(rgb_pixel: RgbaPixel, x: usize, y: usize) -> u8,
}

impl RgbaPixelManipulationProcessor {
    pub fn new(
        red_function: fn(rgb_pixel: RgbaPixel, x: usize, y: usize) -> u8,
        green_function: fn(rgb_pixel: RgbaPixel, x: usize, y: usize) -> u8,
        blue_function: fn(rgb_pixel: RgbaPixel, x: usize, y: usize) -> u8,
        alpha_function: fn(rgb_pixel: RgbaPixel, x: usize, y: usize) -> u8,
    ) -> RgbaPixelManipulationProcessor {
        RgbaPixelManipulationProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            channel_selector: ChannelSelector::Rgba(Default::default()),
            red_function,
            green_function,
            blue_function,
            alpha_function,
        }
    }

    fn run_multi_thread_cpu(
        &self,
        mut fast_image: FastImage,
        cpu_options: CpuOptions,
    ) -> FastImage {
        let run_red = self.channel_selector.red_enabled();
        let run_green = self.channel_selector.green_enabled();
        let run_blue = self.channel_selector.blue_enabled();
        let run_alpha = self.channel_selector.alpha_enabled();

        let red_function = &self.red_function;
        let green_function = &self.green_function;
        let blue_function = &self.blue_function;
        let alpha_function = &self.alpha_function;

        cpu_options.build_thread_pool().install(|| {
            fast_image.iterate_par_rgba(|pixel, x, y| {
                let mut new_pixel = pixel.clone();

                if run_red {
                    new_pixel.red = red_function(*pixel, x, y);
                }
                if run_green {
                    new_pixel.green = green_function(*pixel, x, y);
                }
                if run_blue {
                    new_pixel.blue = blue_function(*pixel, x, y);
                }
                if run_alpha {
                    new_pixel.alpha = alpha_function(*pixel, x, y);
                }

                *pixel = new_pixel;
            });
        });

        fast_image
    }

    fn run_gpu(&self, fast_image: FastImage) -> FastImage {
        unimplemented!()
    }
}

impl Processor for RgbaPixelManipulationProcessor {
    fn set_execution_plan(&mut self, execution_plan: ExecutionPlan) -> PicturifyResult<()> {
        self.execution_plan = execution_plan;
        Ok(())
    }

    fn set_channel_selector(&mut self, channel_selector: ChannelSelector) -> PicturifyResult<()> {
        match channel_selector {
            ChannelSelector::Rgba(_) => {}
            _ => return Err(ProcessingError::InvalidChannelSelector.into()),
        }
        self.channel_selector = channel_selector;
        Ok(())
    }

    fn process(&self, fast_image: FastImage) -> FastImage {
        match self.execution_plan {
            ExecutionPlan::Cpu(options) => {
                self.run_multi_thread_cpu(fast_image, options)
            }
            ExecutionPlan::Gpu => self.run_gpu(fast_image),
        }
    }
}
