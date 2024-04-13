use crate::common::execution::{CpuOptions, ExecutionPlan, Processor};
use picturify_core::error::PicturifyResult;
use picturify_core::fast_image::apply_fn_to_pixels::{
    ApplyFnToImagePixels,
};
use picturify_core::fast_image::FastImage;
use picturify_core::rayon::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct SobelProcessorOptions {}



pub struct SobelProcessor {
    execution_plan: ExecutionPlan,
    _options: SobelProcessorOptions,
}

const SOBEL_KERNEL_X: [[f32; 3]; 3] = [[-1.0, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]];

const SOBEL_KERNEL_Y: [[f32; 3]; 3] = [[-1.0, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]];

impl Default for SobelProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl SobelProcessor {
    pub fn new() -> SobelProcessor {
        SobelProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            _options: Default::default(),
        }
    }

    pub fn with_options(options: SobelProcessorOptions) -> SobelProcessor {
        SobelProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            _options: options,
        }
    }

    fn run_cpu(&self, mut fast_image: FastImage, cpu_options: CpuOptions) -> FastImage {
        let width = fast_image.get_width();
        let height = fast_image.get_height();

        cpu_options.build_thread_pool().install(|| {
            let mut magnitude_vec = vec![vec![0.0; width - 2]; height - 2];
            let min_magnitude = Arc::new(Mutex::new(f32::MAX));
            let max_magnitude = Arc::new(Mutex::new(f32::MIN));

            magnitude_vec
                .iter_mut()
                .enumerate()
                .par_bridge()
                .for_each(|(y_mag, row)| {
                    let mut row_min_magnitude = f32::MAX;
                    let mut row_max_magnitude = f32::MIN;
                    row.iter_mut().enumerate().for_each(|(x_mag, magnitude)| {
                        let x = x_mag + 1;
                        let y = y_mag + 1;

                        let mut magnitude_x = 0.0;
                        let mut magnitude_y = 0.0;

                        for i in 0..3 {
                            for j in 0..3 {
                                let pixel = fast_image.get_image_pixel(x + i - 1, y + j - 1);
                                let red = pixel[0] as f32 / 255.0;
                                let green = pixel[1] as f32 / 255.0;
                                let blue = pixel[2] as f32 / 255.0;

                                magnitude_x += SOBEL_KERNEL_X[j][i] * (red + green + blue) / 3.0;
                                magnitude_y += SOBEL_KERNEL_Y[j][i] * (red + green + blue) / 3.0;
                            }
                        }

                        let actual_magnitude = (magnitude_x.powi(2) + magnitude_y.powi(2)).sqrt();
                        *magnitude = actual_magnitude;

                        if actual_magnitude < row_min_magnitude {
                            row_min_magnitude = actual_magnitude;
                        }

                        if actual_magnitude > row_max_magnitude {
                            row_max_magnitude = actual_magnitude;
                        }
                    });

                    if row_min_magnitude < *min_magnitude.lock().unwrap() {
                        *min_magnitude.lock().unwrap() = row_min_magnitude;
                    }

                    if row_max_magnitude > *max_magnitude.lock().unwrap() {
                        *max_magnitude.lock().unwrap() = row_max_magnitude;
                    }
                });

            let min_magnitude = *min_magnitude.lock().unwrap();
            let max_magnitude = *max_magnitude.lock().unwrap();

            fast_image.apply_fn_to_image_pixel(|pixel, x, y| {
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    return;
                }
                let magnitude = magnitude_vec[y - 1][x - 1];
                let magnitude =
                    ((magnitude - min_magnitude) / (max_magnitude - min_magnitude)) * 255.0;
                let magnitude = magnitude as u8;

                pixel[0] = magnitude;
                pixel[1] = magnitude;
                pixel[2] = magnitude;
            });

            fast_image
        })
    }

    fn run_gpu(&self, _fast_image: FastImage) -> FastImage {
        unimplemented!()
    }
}

impl Processor for SobelProcessor {
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
