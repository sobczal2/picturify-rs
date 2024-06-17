use crate::common::kernels::xy::XyKernels;
use crate::common::processors::CpuProcessor;
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

pub struct GradientBasedProcessorOptions {
    pub use_fast_approximation: bool,
    pub xy_kernels: XyKernels,
}

pub struct GradientBasedProcessor {
    options: GradientBasedProcessorOptions,
}

impl GradientBasedProcessor {
    pub fn new(options: GradientBasedProcessorOptions) -> Result<Self, ProcessingPicturifyError> {
        Ok(Self { options })
    }
}

impl CpuProcessor for GradientBasedProcessor {
    fn process(
        &self,
        mut image: FastImage,
        mut progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        let (width, height): (usize, usize) = image.size().into();
        let kernel_radius = self.options.xy_kernels.radius();

        let mut magnitude_vec =
            vec![vec![0.0; width - 2 * kernel_radius]; height - 2 * kernel_radius];
        let min_magnitude = Arc::new(Mutex::new(f32::MAX));
        let max_magnitude = Arc::new(Mutex::new(f32::MIN));

        let kernels = self.options.xy_kernels.clone();

        let get_pixel_fn: fn(image: &FastImage, coord: Coord) -> Box<dyn RgbaF32Pixel> =
            if self.options.use_fast_approximation {
                |image, coord| Box::new(FastImage::get_image_pixel(image, coord))
            } else {
                |image, coord| Box::new(FastImage::get_lin_srgba_pixel(image, coord))
            };

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
                    let pixel_coord: Coord = (x_mag + kernel_radius, y_mag + kernel_radius).into();

                    let mut magnitude_x = 0.0;
                    let mut magnitude_y = 0.0;

                    kernels.iter().for_each(|(coord, x_value, y_value)| {
                        let actual_coord = (pixel_coord + coord) - kernel_radius as i32;

                        let pixel = get_pixel_fn(&image, actual_coord);

                        let colors = pixel.red_f32() + pixel.green_f32() + pixel.blue_f32();

                        magnitude_x += x_value * colors;
                        magnitude_y += y_value * colors;
                    });

                    magnitude_x /= 3.0;
                    magnitude_y /= 3.0;

                    let actual_magnitude = (magnitude_x.powi(2) + magnitude_y.powi(2)).sqrt();
                    *magnitude = actual_magnitude;

                    row_min_magnitude = row_min_magnitude.min(actual_magnitude);
                    row_max_magnitude = row_max_magnitude.max(actual_magnitude);
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

        Ok(image)
    }
}
