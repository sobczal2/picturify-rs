use crate::common::execution::{Processor, WithOptions};
use picturify_core::fast_image::apply_fn_to_pixels::ApplyFnToPalettePixels;
use picturify_core::fast_image::FastImage;
use picturify_core::fast_image::util::{cord_2d_to_1d, image_rgba_to_palette_srgba};
use picturify_core::palette::{Hsva, IntoColor};
use std::ops::Range;
use std::sync::{Arc, RwLock};
use picturify_core::threading::progress::Progress;

pub struct KuwaharaProcessorOptions {
    pub radius: usize,
}

impl Default for KuwaharaProcessorOptions {
    fn default() -> KuwaharaProcessorOptions {
        KuwaharaProcessorOptions { radius: 3 }
    }
}

pub struct KuwaharaProcessor {
    options: KuwaharaProcessorOptions,
}

impl KuwaharaProcessor {
    pub fn new() -> Self {
        KuwaharaProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<KuwaharaProcessorOptions> for KuwaharaProcessor {
    fn with_options(self, options: KuwaharaProcessorOptions) -> Self {
        KuwaharaProcessor { options }
    }
}

impl Processor for KuwaharaProcessor {
    fn process(&self, mut fast_image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage {
        let width = fast_image.get_width();
        let height = fast_image.get_height();

        let radius = self.options.radius;

        let mut value_array = vec![0.0; width * height];

        value_array
            .iter_mut()
            .zip(fast_image.pixels())
            .for_each(|(value, pixel)| {
                let rgba = image_rgba_to_palette_srgba(*pixel);
                let hsva: Hsva = rgba.into_color();
                *value = hsva.value;
            });

        fast_image.par_apply_fn_to_pixel(
            |pixel: Hsva, x, y| {
                if x < radius || y < radius || x >= width - radius || y >= height - radius {
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
            },
            Some(progress),
        );

        fast_image
    }
}

fn calculate_variance(
    value_vec: &[f32],
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

    (sum_squared - mean * mean) / count as f32
}

fn calculate_mean(
    value_vec: &[f32],
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


    sum / count as f32
}