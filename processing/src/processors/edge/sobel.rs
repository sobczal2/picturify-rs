use picturify_core::error::PicturifyResult;
use picturify_core::image::fast_image::FastImage;
use picturify_core::image::layer::LayerType;
use picturify_core::image::pixel::RgbaPixel;
use picturify_core::image::virtual_image::{VirtualHslImage, VirtualHsvaImage, VirtualImage, VirtualRgbaImage};

use crate::common::channel::{ChannelSelector, HslaChannelSelector, HsvaChannelSelector, LaChannelSelector, RgbaChannelSelector};
use crate::common::execution::{CpuOptions, ExecutionPlan};
use crate::common::process::Processor;

pub struct SobelOperatorProcessor {
    execution_plan: ExecutionPlan,
    channel_selector: ChannelSelector,
    options: SobelOperatorProcessorOptions,
}

pub struct SobelOperatorProcessorOptions {
    pub magnitude_mapping: fn(RgbaPixel, f32) -> RgbaPixel,
}

impl SobelOperatorProcessor {
    pub fn new() -> SobelOperatorProcessor {
        SobelOperatorProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            channel_selector: ChannelSelector::Rgba(Default::default()),
            options: SobelOperatorProcessorOptions {
                magnitude_mapping: |old_pixel, magnitude_squared| {
                    let magnitude = (magnitude_squared.sqrt() * 255.0).min(255.0).round() as u8;
                    RgbaPixel {
                        red: magnitude,
                        green: magnitude,
                        blue: magnitude,
                        alpha: old_pixel.alpha,
                    }
                },
            },
        }
    }

    pub fn change_options(&mut self, action: fn(&mut SobelOperatorProcessorOptions)) {
        action(&mut self.options);
    }

    fn run_cpu(&self, fast_image: FastImage, cpu_options: CpuOptions) -> FastImage {
        cpu_options.build_thread_pool().install(|| {
            return match self.channel_selector {
                ChannelSelector::Rgba(selector) => {
                    self.run_cpu_rgba(fast_image, selector)
                }
                ChannelSelector::Hsva(selector) => {
                    self.run_cpu_hsva(fast_image, selector)
                }
                ChannelSelector::Hsla(selector) => {
                    self.run_cpu_hsla(fast_image, selector)
                }
                ChannelSelector::La(selector) => {
                    self.run_cpu_la(fast_image, selector)
                }
            }
        })
    }

    fn run_gpu(&self, _fast_image: FastImage) -> FastImage {
        unimplemented!()
    }

    fn get_sobel_magnitude_squared(fast_image: &FastImage, layer_type: LayerType, x: usize, y: usize) -> f32 {
        let mut sobel_x = 0.0;
        let mut sobel_y = 0.0;

        let value_00 = fast_image.get_normalized_value(&layer_type, x - 1, y - 1);
        let value_01 = fast_image.get_normalized_value(&layer_type, x - 1, y);
        let value_02 = fast_image.get_normalized_value(&layer_type, x - 1, y + 1);
        let value_10 = fast_image.get_normalized_value(&layer_type, x, y - 1);
        let value_12 = fast_image.get_normalized_value(&layer_type, x, y + 1);
        let value_20 = fast_image.get_normalized_value(&layer_type, x + 1, y - 1);
        let value_21 = fast_image.get_normalized_value(&layer_type, x + 1, y);
        let value_22 = fast_image.get_normalized_value(&layer_type, x + 1, y + 1);

        sobel_x -= value_00;
        sobel_x -= value_01 * 2.0;
        sobel_x -= value_02;
        sobel_x += value_20;
        sobel_x += value_21 * 2.0;
        sobel_x += value_22;

        sobel_y -= value_00;
        sobel_y -= value_10 * 2.0;
        sobel_y -= value_20;
        sobel_y += value_02;
        sobel_y += value_12 * 2.0;
        sobel_y += value_22;

        sobel_x.powi(2) + sobel_y.powi(2)
    }
}

impl Processor for SobelOperatorProcessor {
    fn set_execution_plan(&mut self, execution_plan: ExecutionPlan) -> PicturifyResult<()> {
        self.execution_plan = execution_plan;
        Ok(())
    }

    fn set_channel_selector(&mut self, channel_selector: ChannelSelector) -> PicturifyResult<()> {
        self.channel_selector = channel_selector;
        Ok(())
    }

    fn process(&self, fast_image: FastImage) -> FastImage {
        match self.execution_plan {
            ExecutionPlan::Cpu(options) => {
                return self.run_cpu(fast_image, options);
            }
            ExecutionPlan::Gpu => {
                return self.run_gpu(fast_image);
            }
        }
    }
}

impl SobelOperatorProcessor {
    fn run_cpu_rgba(&self, fast_image: FastImage, rgba_channel_selector: RgbaChannelSelector) -> FastImage {
        let width = fast_image.get_width();
        let height = fast_image.get_height();

        let mut output_image = FastImage::empty(width, height);

        output_image.iterate_par_rgba(|pixel, x, y| {
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                return;
            }
            let mut magnitude_sum = 0.0;
            let mut magnitude_count = 0;

            if rgba_channel_selector.red_enabled() {
                let r = Self::get_sobel_magnitude_squared(&fast_image, LayerType::Red, x, y);
                magnitude_sum += r;
                magnitude_count += 1;
            }
            if rgba_channel_selector.green_enabled() {
                let g = Self::get_sobel_magnitude_squared(&fast_image, LayerType::Green, x, y);
                magnitude_sum += g;
                magnitude_count += 1;
            }
            if rgba_channel_selector.blue_enabled() {
                let b = Self::get_sobel_magnitude_squared(&fast_image, LayerType::Blue, x, y);
                magnitude_sum += b;
                magnitude_count += 1;
            }
            if rgba_channel_selector.alpha_enabled() {
                let a = Self::get_sobel_magnitude_squared(&fast_image, LayerType::Alpha, x, y);
                magnitude_sum += a;
                magnitude_count += 1;
            }

            let old_pixel = fast_image.get_rgba(x, y);
            let new_pixel = (self.options.magnitude_mapping)(old_pixel, magnitude_sum / magnitude_count as f32);

            pixel.red = new_pixel.red;
            pixel.green = new_pixel.green;
            pixel.blue = new_pixel.blue;
            pixel.alpha = new_pixel.alpha;
        });

        output_image
    }

    fn run_cpu_hsva(&self, fast_image: FastImage, hsva_channel_selector: HsvaChannelSelector) -> FastImage {
        let width = fast_image.get_width();
        let height = fast_image.get_height();

        let mut output_image = FastImage::empty(width, height);

        output_image.iterate_par_hsva(|pixel, x, y| {
            if x == 0 || y == 0 || x == fast_image.get_width() - 1 || y == fast_image.get_height() - 1 {
                return;
            }
            let mut magnitude_sum = 0.0;
            let mut magnitude_count = 0;

            if hsva_channel_selector.hue_enabled() {
                let h = Self::get_sobel_magnitude_squared(&fast_image, LayerType::Hue, x, y);
                magnitude_sum += h;
                magnitude_count += 1;
            }
            if hsva_channel_selector.saturation_enabled() {
                let s = Self::get_sobel_magnitude_squared(&fast_image, LayerType::Saturation, x, y);
                magnitude_sum += s;
                magnitude_count += 1;
            }
            if hsva_channel_selector.value_enabled() {
                let v = Self::get_sobel_magnitude_squared(&fast_image, LayerType::Value, x, y);
                magnitude_sum += v;
                magnitude_count += 1;
            }
            if hsva_channel_selector.alpha_enabled() {
                let a = Self::get_sobel_magnitude_squared(&fast_image, LayerType::Alpha, x, y);
                magnitude_sum += a;
                magnitude_count += 1;
            }
            let old_pixel = fast_image.get_rgba(x, y);
            let new_pixel = (self.options.magnitude_mapping)(old_pixel, magnitude_sum / magnitude_count as f32);

            pixel.copy_from_rgba(new_pixel);
        });

        output_image
    }

    fn run_cpu_hsla(&self, fast_image: FastImage, hsla_channel_selector: HslaChannelSelector) -> FastImage {
        let width = fast_image.get_width();
        let height = fast_image.get_height();

        let mut output_image = FastImage::empty(width, height);

        output_image.iterate_par_hsla(|pixel, x, y| {
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                return;
            }
            let mut magnitude_sum = 0.0;
            let mut magnitude_count = 0;

            if hsla_channel_selector.hue_enabled() {
                let h = Self::get_sobel_magnitude_squared(&fast_image, LayerType::Hue, x, y);
                magnitude_sum += h;
                magnitude_count += 1;
            }
            if hsla_channel_selector.saturation_enabled() {
                let s = Self::get_sobel_magnitude_squared(&fast_image, LayerType::Saturation, x, y);
                magnitude_sum += s;
                magnitude_count += 1;
            }
            if hsla_channel_selector.lightness_enabled() {
                let l = Self::get_sobel_magnitude_squared(&fast_image, LayerType::Lightness, x, y);
                magnitude_sum += l;
                magnitude_count += 1;
            }
            if hsla_channel_selector.alpha_enabled() {
                let a = Self::get_sobel_magnitude_squared(&fast_image, LayerType::Alpha, x, y);
                magnitude_sum += a;
                magnitude_count += 1;
            }
            let old_pixel = fast_image.get_rgba(x, y);
            let new_pixel = (self.options.magnitude_mapping)(old_pixel, magnitude_sum / magnitude_count as f32);

            pixel.copy_from_rgba(new_pixel);
        });

        output_image
    }

    fn run_cpu_la(&self, _fast_image: FastImage, _la_channel_selector: LaChannelSelector) -> FastImage {
        unimplemented!()
    }
}