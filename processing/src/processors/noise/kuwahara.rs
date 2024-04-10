use crate::common::execution::{CpuOptions, ExecutionPlan, Processor};
use picturify_core::error::PicturifyResult;
use picturify_core::fast_image::apply_fn_to_pixels::ApplyFnToPalettePixels;
use picturify_core::fast_image::fast_image::FastImage;
use picturify_core::fast_image::util::{cord_2d_to_1d, image_rgba_to_palette_srgba};
use std::ops::Range;
use picturify_core::palette::{Hsva, IntoColor};

pub struct KuwaharaProcessorOptions {
    pub radius: usize,
}

impl Default for KuwaharaProcessorOptions {
    fn default() -> KuwaharaProcessorOptions {
        KuwaharaProcessorOptions { radius: 3 }
    }
}

pub struct KuwaharaProcessor {
    execution_plan: ExecutionPlan,
    options: KuwaharaProcessorOptions,
}

impl KuwaharaProcessor {
    pub fn new() -> KuwaharaProcessor {
        KuwaharaProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options: Default::default(),
        }
    }

    pub fn with_options(options: KuwaharaProcessorOptions) -> KuwaharaProcessor {
        KuwaharaProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options,
        }
    }

    fn run_cpu(&self, mut fast_image: FastImage, cpu_options: CpuOptions) -> FastImage {
        let width = fast_image.get_width();
        let height = fast_image.get_height();

        let radius = self.options.radius;

        cpu_options.build_thread_pool().install(|| {
            let mut value_array = vec![0.0; width * height];

            value_array
                .iter_mut()
                .zip(fast_image.pixels())
                .for_each(|(value, pixel)| {
                    let rgba = image_rgba_to_palette_srgba(*pixel);
                    let hsva: Hsva = rgba.into_color();
                    *value = hsva.value;
                });

            fast_image.par_apply_fn_to_pixel(|pixel: Hsva, x, y| {
                if x < radius
                    || y < radius
                    || x >= width - radius
                    || y >= height - radius
                {
                    return pixel;
                }

                let quadrant1_ranges = (x - radius..x, y - radius..y);
                let quadrant2_ranges = (x..x + radius, y - radius..y);
                let quadrant3_ranges = (x - radius..x, y..y + radius);
                let quadrant4_ranges = (x..x + radius, y..y + radius);

                let quadrant_1_variance = calculate_variance(
                    &value_array,
                    &quadrant1_ranges.0,
                    &quadrant1_ranges.1,
                    width,
                );
                let quadrant_2_variance = calculate_variance(
                    &value_array,
                    &quadrant2_ranges.0,
                    &quadrant2_ranges.1,
                    width,
                );
                let quadrant_3_variance = calculate_variance(
                    &value_array,
                    &quadrant3_ranges.0,
                    &quadrant3_ranges.1,
                    width,
                );
                let quadrant_4_variance = calculate_variance(
                    &value_array,
                    &quadrant4_ranges.0,
                    &quadrant4_ranges.1,
                    width,
                );

                let min_quadrant = [
                    quadrant_1_variance,
                    quadrant_2_variance,
                    quadrant_3_variance,
                    quadrant_4_variance,
                ]
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap()
                .0;

                let (range_x, range_y) = match min_quadrant {
                    0 => quadrant1_ranges,
                    1 => quadrant2_ranges,
                    2 => quadrant3_ranges,
                    3 => quadrant4_ranges,
                    _ => unreachable!(),
                };

                let mean = calculate_mean(&value_array, &range_x, &range_y, width);

                Hsva::new(pixel.hue, pixel.saturation, mean, pixel.alpha)
            });

            fast_image
        })
    }

    fn run_gpu(&self, _fast_image: FastImage) -> FastImage {
        unimplemented!()
    }
}

impl Processor for KuwaharaProcessor {
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

fn calculate_variance(
    value_vec: &Vec<f32>,
    range_x: &Range<usize>,
    range_y: &Range<usize>,
    width: usize,
) -> f32 {
    let mut sum = 0.0;
    let mut sum_squared = 0.0;
    let mut count = 0;
    for x in range_x.clone() {
        for y in range_y.clone() {
            let value = value_vec[cord_2d_to_1d(x, y, width)];
            sum += value;
            sum_squared += value * value;
            count += 1;
        }
    }

    let mean = sum / count as f32;
    let variance = (sum_squared - mean * mean) / count as f32;
    variance
}

fn calculate_mean(
    value_vec: &Vec<f32>,
    range_x: &Range<usize>,
    range_y: &Range<usize>,
    width: usize,
) -> f32 {
    let mut sum = 0.0;
    let mut count = 0;
    for x in range_x.clone() {
        for y in range_y.clone() {
            let value = value_vec[cord_2d_to_1d(x, y, width)];
            sum += value;
            count += 1;
        }
    }

    let mean = sum / count as f32;
    mean
}
