use crate::common::execution::Processor;
use crate::common::kernels::xy::XyKernels;
use picturify_core::core::apply_fn_to_pixels::{
    ApplyFnToImagePixels, ApplyFnToPalettePixels, Offset,
};
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::{ProcessingPicturifyError, ProcessingPicturifyResult};
use picturify_core::geometry::coord::Coord;
use picturify_core::pixel::traits::RgbaF32Pixel;
use picturify_core::rayon::prelude::*;
use picturify_core::threading::progress::{Progress, ProgressIteratorExt};
use std::sync::{Arc, Mutex};

pub struct GradientBasedRgbProcessorOptions {
    pub use_fast_approximation: bool,
    pub xy_kernels: XyKernels,
}

pub struct GradientBasedRgbProcessor {
    options: GradientBasedRgbProcessorOptions,
}

impl GradientBasedRgbProcessor {
    pub fn new(
        options: GradientBasedRgbProcessorOptions,
    ) -> Result<Self, ProcessingPicturifyError> {
        Ok(Self { options })
    }
}

impl Processor for GradientBasedRgbProcessor {
    fn process(
        &self,
        mut image: FastImage,
        mut progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let (width, height): (usize, usize) = image.size().into();
        let kernel_radius = self.options.xy_kernels.radius();

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

        let kernels = self.options.xy_kernels.clone();

        let get_pixel_fn: fn(image: &FastImage, coord: Coord) -> Box<dyn RgbaF32Pixel> =
            if self.options.use_fast_approximation {
                |image, coord| Box::new(FastImage::get_image_pixel(image, coord))
            } else {
                |image, coord| Box::new(FastImage::get_lin_srgba_pixel(image, coord))
            };

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
                            let pixel_coord: Coord =
                                (x_mag + kernel_radius, y_mag + kernel_radius).into();

                            let mut red_magnitude_x = 0.0;
                            let mut red_magnitude_y = 0.0;
                            let mut green_magnitude_x = 0.0;
                            let mut green_magnitude_y = 0.0;
                            let mut blue_magnitude_x = 0.0;
                            let mut blue_magnitude_y = 0.0;

                            kernels.iter().for_each(|(coord, x_value, y_value)| {
                                let actual_coord = (pixel_coord + coord) - kernel_radius as i32;

                                let pixel = get_pixel_fn(&image, actual_coord);

                                let red = pixel.red_f32();
                                let green = pixel.green_f32();
                                let blue = pixel.blue_f32();

                                red_magnitude_x += x_value * red;
                                red_magnitude_y += y_value * red;
                                green_magnitude_x += x_value * green;
                                green_magnitude_y += y_value * green;
                                blue_magnitude_x += x_value * blue;
                                blue_magnitude_y += y_value * blue;
                            });

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

        Ok(image)
    }
}
