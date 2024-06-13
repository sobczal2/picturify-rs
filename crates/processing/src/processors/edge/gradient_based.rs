use crate::common::execution::Processor;
use picturify_core::error::processing::ProcessingError;
use picturify_core::fast_image::apply_fn_to_pixels::{
    ApplyFnToImagePixels, ApplyFnToPalettePixels, Offset,
};
use picturify_core::fast_image::FastImage;
use picturify_core::rayon::prelude::*;
use picturify_core::threading::progress::{Progress, ProgressIteratorExt};
use std::sync::{Arc, Mutex};

pub struct GradientBasedProcessorOptions {
    pub use_fast_approximation: bool,
    pub x_kernel: Vec<Vec<f32>>,
    pub y_kernel: Vec<Vec<f32>>,
}

pub struct GradientBasedProcessor {
    options: GradientBasedProcessorOptions,
}

impl GradientBasedProcessor {
    pub fn new(options: GradientBasedProcessorOptions) -> Result<Self, ProcessingError> {
        if !GradientBasedProcessor::are_kernels_valid(&options) {
            return Err(ProcessingError::InvalidKernel);
        }

        Ok(Self { options })
    }

    fn are_kernels_valid(options: &GradientBasedProcessorOptions) -> bool {
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

impl Processor for GradientBasedProcessor {
    fn process(&self, mut image: FastImage, mut progress: Progress) -> FastImage {
        let (width, height): (usize, usize) = image.size().into();
        let kernel_radius = self.options.x_kernel.len() / 2;

        let mut magnitude_vec =
            vec![vec![0.0; width - 2 * kernel_radius]; height - 2 * kernel_radius];
        let min_magnitude = Arc::new(Mutex::new(f32::MAX));
        let max_magnitude = Arc::new(Mutex::new(f32::MIN));

        let x_kernel = self.options.x_kernel.clone();
        let y_kernel = self.options.y_kernel.clone();

        let kernel_width = x_kernel.len();
        let kernel_height = x_kernel[0].len();

        progress.setup((height - kernel_radius) * 2);
        magnitude_vec
            .iter_mut()
            .enumerate()
            .progress(progress.clone())
            .par_bridge()
            .for_each(|(y_mag, row)| {
                let mut row_min_magnitude = f32::MAX;
                let mut row_max_magnitude = f32::MIN;
                row.iter_mut().enumerate().for_each(|(x_mag, magnitude)| {
                    let x = x_mag + kernel_radius;
                    let y = y_mag + kernel_radius;

                    let mut magnitude_x = 0.0;
                    let mut magnitude_y = 0.0;

                    for i in 0..kernel_height {
                        for j in 0..kernel_width {
                            let red: f32;
                            let green: f32;
                            let blue: f32;

                            let coord = (x + i - kernel_radius, y + j - kernel_radius).into();

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

                            magnitude_x += x_kernel[j][i] * (red + green + blue) / 3.0;
                            magnitude_y += y_kernel[j][i] * (red + green + blue) / 3.0;
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

        let mut inner_progress = Progress::new();
        inner_progress.set_on_increment(move || {
            progress.increment();
        });

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

                    let magnitude = magnitude_vec[y][x];
                    let magnitude =
                        ((magnitude - min_magnitude) / (max_magnitude - min_magnitude)) * 255.0;
                    let magnitude = magnitude as u8;

                    pixel[0] = magnitude;
                    pixel[1] = magnitude;
                    pixel[2] = magnitude;
                },
                Some(inner_progress),
                offset,
            );
        } else {
            image.par_apply_fn_to_lin_srgba_with_offset(
                |mut pixel, coord| {
                    let (x, y): (usize, usize) = (coord + coord_offset).into();

                    let magnitude = magnitude_vec[y][x];
                    let magnitude = (magnitude - min_magnitude) / (max_magnitude - min_magnitude);

                    pixel.red = magnitude;
                    pixel.green = magnitude;
                    pixel.blue = magnitude;

                    pixel
                },
                Some(inner_progress),
                offset,
            );
        }

        image
    }
}
