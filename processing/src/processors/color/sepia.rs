use picturify_core::error::processing::ProcessingError;
use picturify_core::error::PicturifyResult;
use picturify_core::image::fast_image::FastImage;
use picturify_core::image::pixel::RgbaPixel;
use picturify_core::image::virtual_image::VirtualRgbaImage;

use crate::common::channel::ChannelSelector;
use crate::common::execution::{CpuOptions, ExecutionPlan};
use crate::common::process::Processor;

pub struct SepiaProcessor {
    execution_plan: ExecutionPlan,
}

impl SepiaProcessor {
    pub fn new() -> SepiaProcessor {
        SepiaProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
        }
    }

    #[inline(always)]
    fn calculate_pixel(&self, pixel: &mut RgbaPixel) {
        let r = pixel.red as f32;
        let g = pixel.green as f32;
        let b = pixel.blue as f32;

        let new_r = (r * 0.393 + g * 0.769 + b * 0.189).min(255.0).round() as u8;
        let new_g = (r * 0.349 + g * 0.686 + b * 0.168).min(255.0).round() as u8;
        let new_b = (r * 0.272 + g * 0.534 + b * 0.131).min(255.0).round() as u8;

        pixel.red = new_r;
        pixel.green = new_g;
        pixel.blue = new_b;
    }

    fn run_cpu(
        &self,
        mut fast_image: FastImage,
        cpu_options: CpuOptions,
    ) -> FastImage {
        cpu_options.build_thread_pool().install(|| {
            fast_image.iterate_par_rgba(|pixel, _x, _y| {
                self.calculate_pixel(pixel);
            });
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

    fn set_channel_selector(&mut self, _channel_selector: ChannelSelector) -> PicturifyResult<()> {
        Err(ProcessingError::ChannelSelectionNotSupported.into())
    }

    fn process(&self, fast_image: FastImage) -> FastImage {
        match self.execution_plan {
            ExecutionPlan::Cpu(options) => self.run_cpu(fast_image, options),
            ExecutionPlan::Gpu => self.run_gpu(fast_image),
        }
    }
}
