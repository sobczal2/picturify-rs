use std::collections::VecDeque;
use picturify_core::error::PicturifyResult;
use picturify_core::fast_image::fast_image::FastImage;
use picturify_core::rayon::prelude::*;
use crate::common::execution::{CpuOptions, ExecutionPlan, Processor};

pub struct MedianProcessorOptions {
    pub radius: usize,
}

impl Default for MedianProcessorOptions {
    fn default() -> MedianProcessorOptions {
        MedianProcessorOptions { radius: 3 }
    }
}

pub struct MedianProcessor {
    execution_plan: ExecutionPlan,
    options: MedianProcessorOptions,
}

impl MedianProcessor {
    pub fn new() -> MedianProcessor {
        MedianProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options: Default::default(),
        }
    }

    pub fn with_options(options: MedianProcessorOptions) -> MedianProcessor {
        MedianProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options,
        }
    }

    fn run_cpu(&self, fast_image: FastImage, cpu_options: CpuOptions) -> FastImage {
        let width = fast_image.get_width();
        let height = fast_image.get_height();
        let radius = self.options.radius;

        cpu_options.build_thread_pool().install(|| {
            let mut new_fast_image = fast_image.clone();

            new_fast_image.rows_mut().enumerate().skip(radius).take(height - 2 * radius).par_bridge().for_each(|(y, row)| {
                let mut red_window = VecDeque::with_capacity((2 * radius + 1) * (2 * radius + 1));
                let mut green_window = VecDeque::with_capacity((2 * radius + 1) * (2 * radius + 1));
                let mut blue_window = VecDeque::with_capacity((2 * radius + 1) * (2 * radius + 1));

                let radius_i32 = radius as i32;
                for window_x in -radius_i32..=radius_i32 {
                    for window_y in -radius_i32..=radius_i32 {
                        let pixel = fast_image.get_image_pixel((radius_i32 + window_x) as usize, (y as i32 + window_y) as usize);
                        red_window.push_back(pixel[0]);
                        green_window.push_back(pixel[1]);
                        blue_window.push_back(pixel[2]);
                    }
                }
                row.into_iter().enumerate().skip(radius).take(width - 2 * radius).for_each(|(x, pixel)| {
                    let current_red_median = calculate_median(&mut red_window);
                    let current_green_median = calculate_median(&mut green_window);
                    let current_blue_median = calculate_median(&mut blue_window);

                    pixel[0] = current_red_median;
                    pixel[1] = current_green_median;
                    pixel[2] = current_blue_median;

                    for window_y in -radius_i32..=radius_i32 {
                        let pixel = fast_image.get_image_pixel((x as i32 - radius_i32) as usize, (y as i32 + window_y) as usize);
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
        })
    }

    fn run_gpu(&self, _fast_image: FastImage) -> FastImage {
        unimplemented!()
    }
}

impl Processor for MedianProcessor {
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