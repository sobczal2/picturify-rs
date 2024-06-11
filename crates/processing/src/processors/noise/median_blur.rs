use crate::common::execution::{Processor, WithOptions};
use picturify_core::fast_image::FastImage;
use picturify_core::rayon::prelude::*;
use picturify_core::threading::progress::{Progress, ProgressIteratorExt};
use std::collections::VecDeque;

pub struct MedianBlurProcessorOptions {
    pub radius: usize,
}

impl Default for MedianBlurProcessorOptions {
    fn default() -> MedianBlurProcessorOptions {
        MedianBlurProcessorOptions { radius: 3 }
    }
}

pub struct MedianBlurProcessor {
    options: MedianBlurProcessorOptions,
}

impl MedianBlurProcessor {
    pub fn new() -> Self {
        MedianBlurProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<MedianBlurProcessorOptions> for MedianBlurProcessor {
    fn with_options(self, options: MedianBlurProcessorOptions) -> Self {
        MedianBlurProcessor { options }
    }
}

impl Processor for MedianBlurProcessor {
    fn process(&self, image: FastImage, mut progress: Progress) -> FastImage {
        let (width, height): (usize, usize) = image.size().into();
        let radius = self.options.radius;

        let mut new_fast_image = image.clone();

        progress.setup(height - 2 * radius);
        new_fast_image
            .rows_mut()
            .enumerate()
            .skip(radius)
            .take(height - 2 * radius)
            .progress(progress)
            .par_bridge()
            .for_each(|(y, row)| {
                let mut red_window = VecDeque::with_capacity((2 * radius + 1) * (2 * radius + 1));
                let mut green_window = VecDeque::with_capacity((2 * radius + 1) * (2 * radius + 1));
                let mut blue_window = VecDeque::with_capacity((2 * radius + 1) * (2 * radius + 1));

                let radius_i32 = radius as i32;
                for window_x in -radius_i32..=radius_i32 {
                    for window_y in -radius_i32..=radius_i32 {
                        let coord = (window_x + radius_i32, window_y + y as i32).into();
                        let pixel = image.get_image_pixel(coord);
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
                            let coord = (x as i32 - radius_i32, y as i32 + window_y).into();
                            let pixel = image.get_image_pixel(coord);
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
