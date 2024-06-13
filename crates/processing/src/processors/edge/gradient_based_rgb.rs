use crate::common::execution::Processor;
use picturify_core::error::processing::ProcessingError;
use picturify_core::fast_image::apply_fn_to_pixels::{
    ApplyFnToImagePixels, ApplyFnToPalettePixels, Offset,
};
use picturify_core::fast_image::FastImage;
use picturify_core::rayon::prelude::*;
use picturify_core::threading::progress::{Progress, ProgressIteratorExt};
use std::sync::{Arc, Mutex};

pub struct GradientBasedRgbProcessorOptions {
    pub use_fast_approximation: bool,
    pub x_kernel: Vec<Vec<f32>>,
    pub y_kernel: Vec<Vec<f32>>,
}

pub struct GradientBasedRgbProcessor {
    options: GradientBasedRgbProcessorOptions,
}

impl GradientBasedRgbProcessor {
    pub fn new(options: GradientBasedRgbProcessorOptions) -> Result<Self, ProcessingError> {
        if !GradientBasedRgbProcessor::are_kernels_valid(&options) {
            return Err(ProcessingError::InvalidKernel);
        }

        Ok(Self { options })
    }

    fn are_kernels_valid(options: &GradientBasedRgbProcessorOptions) -> bool {
        let x_kernel = &options.x_kernel;
        let y_kernel = &options.y_kernel;

        let x_kernel_width = x_kernel.len();
        let y_kernel_width = y_kernel.len();

        if x_kernel_width % 2 == 0 || y_kernel_width % 2 == 0 {
            return false;
        }

        if x_kernel_width != y_kernel_width {
            return false;
        }

        for i in 0..x_kernel_width {
            if x_kernel[i].len() != x_kernel_width || y_kernel[i].len() != y_kernel_width {
                return false;
            }
        }

        true
    }
}

impl Processor for GradientBasedRgbProcessor {
    fn process(&self, mut image: FastImage, mut progress: Progress) -> FastImage {
        let (width, height): (usize, usize) = image.size().into();
        let kernel_radius = self.options.x_kernel.len() / 2;

        let mut red_magnitude_vec =
            vec![vec![0.0; width - 2 * kernel_radius]; height - 2 * kernel_radius];
        let mut green_magnitude_vec =
            vec![vec![0.0; width - 2 * kernel_radius]; height - 2 * kernel_radius];
        let mut blue_magnitude_vec =
            vec![vec![0.0; width - 2 * kernel_radius]; height - 2 * kernel_radius];

        let red_min_magnitude = Arc::new(Mutex::new(f32::MAX));
        let green_min_magnitude = Arc::new(Mutex::new(f32::MAX));
        let blue_min_magnitude = Arc::new(Mutex::new(f32::MAX));

        let red_max_magnitude = Arc::new(Mutex::new(f32::MIN));
        let green_max_magnitude = Arc::new(Mutex::new(f32::MIN));
        let blue_max_magnitude = Arc::new(Mutex::new(f32::MIN));

        let x_kernel = self.options.x_kernel.clone();
        let y_kernel = self.options.y_kernel.clone();

        let kernel_width = x_kernel.len();
        let kernel_height = x_kernel[0].len();

        progress.setup((width - kernel_radius) * 2);
        red_magnitude_vec
            .iter_mut()
            .zip(green_magnitude_vec.iter_mut())
            .zip(blue_magnitude_vec.iter_mut())
            .enumerate()
            .progress(progress.clone())
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
                            let x = x_mag + kernel_radius;
                            let y = y_mag + kernel_radius;

                            let mut red_magnitude_x = 0.0;
                            let mut red_magnitude_y = 0.0;
                            let mut green_magnitude_x = 0.0;
                            let mut green_magnitude_y = 0.0;
                            let mut blue_magnitude_x = 0.0;
                            let mut blue_magnitude_y = 0.0;

                            for i in 0..kernel_height {
                                for j in 0..kernel_width {
                                    let red: f32;
                                    let green: f32;
                                    let blue: f32;

                                    let coord =
                                        (x + i - kernel_radius, y + j - kernel_radius).into();

                                    if self.options.use_fast_approximation {
                                        let pixel = image.get_image_pixel(coord);
                                        red = pixel[0] as f32 / 255.0;
                                        green = pixel[1] as f32 / 255.0;
                                        blue = pixel[2] as f32 / 255.0;
                                    } else {
                                        let pixel = image.get_lin_srgba_pixel(coord);
                                        red = pixel.red;
                                        green = pixel.green;
                                        blue = pixel.blue;
                                    }

                                    red_magnitude_x += x_kernel[j][i] * red;
                                    red_magnitude_y += y_kernel[j][i] * red;
                                    green_magnitude_x += x_kernel[j][i] * green;
                                    green_magnitude_y += y_kernel[j][i] * green;
                                    blue_magnitude_x += x_kernel[j][i] * blue;
                                    blue_magnitude_y += y_kernel[j][i] * blue;
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

        let offset = Offset {
            skip_rows: kernel_radius,
            take_rows: height - 2 * kernel_radius,
            skip_columns: kernel_radius,
            take_columns: width - 2 * kernel_radius,
        };

        let coord_offset = (-(kernel_radius as i32), -(kernel_radius as i32)).into();

        if self.options.use_fast_approximation {
            image.par_apply_fn_to_image_pixel_with_offset(
                |pixel, coord| {
                    let (x, y): (usize, usize) = (coord + coord_offset).into();
                    let red_magnitude = red_magnitude_vec[y][x];
                    let green_magnitude = green_magnitude_vec[y][x];
                    let blue_magnitude = blue_magnitude_vec[y][x];
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
                },
                Some(progress),
                offset,
            );
        } else {
            image.par_apply_fn_to_lin_srgba_with_offset(
                |mut pixel, coord| {
                    let (x, y): (usize, usize) = (coord + coord_offset).into();

                    let red_magnitude = red_magnitude_vec[y][x];
                    let green_magnitude = green_magnitude_vec[y][x];
                    let blue_magnitude = blue_magnitude_vec[y][x];
                    let red_magnitude = (red_magnitude - red_min_magnitude)
                        / (red_max_magnitude - red_min_magnitude);
                    let green_magnitude = (green_magnitude - green_min_magnitude)
                        / (green_max_magnitude - green_min_magnitude);
                    let blue_magnitude = (blue_magnitude - blue_min_magnitude)
                        / (blue_max_magnitude - blue_min_magnitude);

                    pixel.red = red_magnitude;
                    pixel.green = green_magnitude;
                    pixel.blue = blue_magnitude;

                    pixel
                },
                Some(progress),
                offset,
            );
        }

        image
    }
}
