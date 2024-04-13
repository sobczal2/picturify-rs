use crate::common::execution::{CpuOptions, ExecutionPlan, Processor};
use picturify_core::error::PicturifyResult;
use picturify_core::fast_image::apply_fn_to_pixels::{
    ApplyFnToImagePixels,
};
use picturify_core::fast_image::FastImage;
use picturify_core::rayon::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct SobelRgbProcessorOptions {}



pub struct SobelRgbProcessor {
    execution_plan: ExecutionPlan,
    _options: SobelRgbProcessorOptions,
}

const SOBEL_KERNEL_X: [[f32; 3]; 3] = [[-1.0, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]];

const SOBEL_KERNEL_Y: [[f32; 3]; 3] = [[-1.0, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]];

impl Default for SobelRgbProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl SobelRgbProcessor {
    pub fn new() -> SobelRgbProcessor {
        SobelRgbProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            _options: Default::default(),
        }
    }

    pub fn with_options(options: SobelRgbProcessorOptions) -> SobelRgbProcessor {
        SobelRgbProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            _options: options,
        }
    }

    fn run_cpu(&self, mut fast_image: FastImage, cpu_options: CpuOptions) -> FastImage {
        let width = fast_image.get_width();
        let height = fast_image.get_height();

        cpu_options.build_thread_pool().install(|| {
            let mut red_magnitude_vec = vec![vec![0.0; width - 2]; height - 2];
            let mut green_magnitude_vec = vec![vec![0.0; width - 2]; height - 2];
            let mut blue_magnitude_vec = vec![vec![0.0; width - 2]; height - 2];
            let red_min_magnitude = Arc::new(Mutex::new(f32::MAX));
            let green_min_magnitude = Arc::new(Mutex::new(f32::MAX));
            let blue_min_magnitude = Arc::new(Mutex::new(f32::MAX));
            let red_max_magnitude = Arc::new(Mutex::new(f32::MIN));
            let green_max_magnitude = Arc::new(Mutex::new(f32::MIN));
            let blue_max_magnitude = Arc::new(Mutex::new(f32::MIN));

            red_magnitude_vec
                .iter_mut()
                .zip(green_magnitude_vec.iter_mut())
                .zip(blue_magnitude_vec.iter_mut())
                .enumerate()
                .par_bridge()
                .for_each(|(y_mag, ((red_row, green_row), blue_row))| {
                    let mut red_row_min_magnitude = f32::MAX;
                    let mut red_row_max_magnitude = f32::MIN;
                    let mut green_row_min_magnitude = f32::MAX;
                    let mut green_row_max_magnitude = f32::MIN;
                    let mut blue_row_min_magnitude = f32::MAX;
                    let mut blue_row_max_magnitude = f32::MIN;
                    red_row
                        .iter_mut()
                        .zip(green_row.iter_mut())
                        .zip(blue_row.iter_mut())
                        .enumerate()
                        .for_each(
                            |(x_mag, ((red_magnitude, green_magnitude), blue_magnitude))| {
                                let x = x_mag + 1;
                                let y = y_mag + 1;

                                let mut red_magnitude_x = 0.0;
                                let mut red_magnitude_y = 0.0;
                                let mut green_magnitude_x = 0.0;
                                let mut green_magnitude_y = 0.0;
                                let mut blue_magnitude_x = 0.0;
                                let mut blue_magnitude_y = 0.0;

                                for i in 0..3 {
                                    for j in 0..3 {
                                        let pixel =
                                            fast_image.get_image_pixel(x + i - 1, y + j - 1);
                                        let red = pixel[0] as f32 / 255.0;
                                        let green = pixel[1] as f32 / 255.0;
                                        let blue = pixel[2] as f32 / 255.0;

                                        red_magnitude_x += SOBEL_KERNEL_X[j][i] * red;
                                        red_magnitude_y += SOBEL_KERNEL_Y[j][i] * red;
                                        green_magnitude_x += SOBEL_KERNEL_X[j][i] * green;
                                        green_magnitude_y += SOBEL_KERNEL_Y[j][i] * green;
                                        blue_magnitude_x += SOBEL_KERNEL_X[j][i] * blue;
                                        blue_magnitude_y += SOBEL_KERNEL_Y[j][i] * blue;
                                    }
                                }

                                let red_actual_magnitude =
                                    (red_magnitude_x.powi(2) + red_magnitude_y.powi(2)).sqrt();
                                let green_actual_magnitude =
                                    (green_magnitude_x.powi(2) + green_magnitude_y.powi(2)).sqrt();
                                let blue_actual_magnitude =
                                    (blue_magnitude_x.powi(2) + blue_magnitude_y.powi(2)).sqrt();

                                *red_magnitude = red_actual_magnitude;
                                *green_magnitude = green_actual_magnitude;
                                *blue_magnitude = blue_actual_magnitude;

                                if red_actual_magnitude < red_row_min_magnitude {
                                    red_row_min_magnitude = red_actual_magnitude;
                                }

                                if red_actual_magnitude > red_row_max_magnitude {
                                    red_row_max_magnitude = red_actual_magnitude;
                                }

                                if green_actual_magnitude < green_row_min_magnitude {
                                    green_row_min_magnitude = green_actual_magnitude;
                                }

                                if green_actual_magnitude > green_row_max_magnitude {
                                    green_row_max_magnitude = green_actual_magnitude;
                                }

                                if blue_actual_magnitude < blue_row_min_magnitude {
                                    blue_row_min_magnitude = blue_actual_magnitude;
                                }

                                if blue_actual_magnitude > blue_row_max_magnitude {
                                    blue_row_max_magnitude = blue_actual_magnitude;
                                }
                            },
                        );

                    if red_row_min_magnitude < *red_min_magnitude.lock().unwrap() {
                        *red_min_magnitude.lock().unwrap() = red_row_min_magnitude;
                    }

                    if red_row_max_magnitude > *red_max_magnitude.lock().unwrap() {
                        *red_max_magnitude.lock().unwrap() = red_row_max_magnitude;
                    }

                    if green_row_min_magnitude < *green_min_magnitude.lock().unwrap() {
                        *green_min_magnitude.lock().unwrap() = green_row_min_magnitude;
                    }

                    if green_row_max_magnitude > *green_max_magnitude.lock().unwrap() {
                        *green_max_magnitude.lock().unwrap() = green_row_max_magnitude;
                    }

                    if blue_row_min_magnitude < *blue_min_magnitude.lock().unwrap() {
                        *blue_min_magnitude.lock().unwrap() = blue_row_min_magnitude;
                    }

                    if blue_row_max_magnitude > *blue_max_magnitude.lock().unwrap() {
                        *blue_max_magnitude.lock().unwrap() = blue_row_max_magnitude;
                    }
                });

            let red_min_magnitude = *red_min_magnitude.lock().unwrap();
            let red_max_magnitude = *red_max_magnitude.lock().unwrap();
            let green_min_magnitude = *green_min_magnitude.lock().unwrap();
            let green_max_magnitude = *green_max_magnitude.lock().unwrap();
            let blue_min_magnitude = *blue_min_magnitude.lock().unwrap();
            let blue_max_magnitude = *blue_max_magnitude.lock().unwrap();

            fast_image.apply_fn_to_image_pixel(|pixel, x, y| {
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    return;
                }
                let red_magnitude = red_magnitude_vec[y - 1][x - 1];
                let green_magnitude = green_magnitude_vec[y - 1][x - 1];
                let blue_magnitude = blue_magnitude_vec[y - 1][x - 1];
                let red_magnitude = ((red_magnitude - red_min_magnitude)
                    / (red_max_magnitude - red_min_magnitude))
                    * 255.0;
                let green_magnitude = ((green_magnitude - green_min_magnitude)
                    / (green_max_magnitude - green_min_magnitude))
                    * 255.0;
                let blue_magnitude = ((blue_magnitude - blue_min_magnitude)
                    / (blue_max_magnitude - blue_min_magnitude))
                    * 255.0;
                let red_magnitude = red_magnitude as u8;
                let green_magnitude = green_magnitude as u8;
                let blue_magnitude = blue_magnitude as u8;

                pixel[0] = red_magnitude;
                pixel[1] = green_magnitude;
                pixel[2] = blue_magnitude;
            });

            fast_image
        })
    }

    fn run_gpu(&self, _fast_image: FastImage) -> FastImage {
        unimplemented!()
    }
}

impl Processor for SobelRgbProcessor {
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
