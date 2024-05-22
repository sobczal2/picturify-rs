use crate::common::execution::{Processor, WithOptions};
use picturify_core::fast_image::FastImage;
use picturify_core::rayon::prelude::*;
use picturify_core::threading::progress::Progress;
use std::collections::VecDeque;

pub struct MedianProcessorOptions {
    pub radius: usize,
}

impl Default for MedianProcessorOptions {
    fn default() -> MedianProcessorOptions {
        MedianProcessorOptions { radius: 3 }
    }
}

pub struct MedianProcessor {
    options: MedianProcessorOptions,
}

impl MedianProcessor {
    pub fn new() -> Self {
        MedianProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<MedianProcessorOptions> for MedianProcessor {
    fn with_options(self, options: MedianProcessorOptions) -> Self {
        MedianProcessor { options }
    }
}

impl Processor for MedianProcessor {
    fn process(&self, image: FastImage, mut progress: Progress) -> FastImage {
        let width = image.get_width();
        let height = image.get_height();
        let radius = self.options.radius;

        let mut new_fast_image = image.clone();

        progress.setup(height - 2 * radius);
        new_fast_image
            .rows_mut()
            .enumerate()
            .skip(radius)
            .take(height - 2 * radius)
            .par_bridge()
            .for_each(|(y, row)| {
                progress.increment();
                let mut red_window = VecDeque::with_capacity((2 * radius + 1) * (2 * radius + 1));
                let mut green_window = VecDeque::with_capacity((2 * radius + 1) * (2 * radius + 1));
                let mut blue_window = VecDeque::with_capacity((2 * radius + 1) * (2 * radius + 1));

                let radius_i32 = radius as i32;
                for window_x in -radius_i32..=radius_i32 {
                    for window_y in -radius_i32..=radius_i32 {
                        let pixel = image.get_image_pixel(
                            (radius_i32 + window_x) as usize,
                            (y as i32 + window_y) as usize,
                        );
                        red_window.push_back(pixel[0]);
                        green_window.push_back(pixel[1]);
                        blue_window.push_back(pixel[2]);
                    }
                }
                row.into_iter()
                    .enumerate()
                    .skip(radius)
                    .take(width - 2 * radius)
                    .for_each(|(x, pixel)| {
                        let current_red_median = calculate_median(&red_window);
                        let current_green_median = calculate_median(&green_window);
                        let current_blue_median = calculate_median(&blue_window);

                        pixel[0] = current_red_median;
                        pixel[1] = current_green_median;
                        pixel[2] = current_blue_median;

                        for window_y in -radius_i32..=radius_i32 {
                            let pixel = image.get_image_pixel(
                                (x as i32 - radius_i32) as usize,
                                (y as i32 + window_y) as usize,
                            );
                            red_window.pop_front();
                            green_window.pop_front();
                            blue_window.pop_front();
                            red_window.push_back(pixel[0]);
                            green_window.push_back(pixel[1]);
                            blue_window.push_back(pixel[2]);
                        }
                    });
            });

        new_fast_image
    }
}

// TODO: Optimize this function
fn calculate_median(values: &VecDeque<u8>) -> u8 {
    let mut values: Vec<u8> = values.iter().copied().collect();
    values.sort_unstable();
    let mid = values.len() / 2;
    if values.len() % 2 == 0 {
        (values[mid - 1] + values[mid]) / 2
    } else {
        values[mid]
    }
}
