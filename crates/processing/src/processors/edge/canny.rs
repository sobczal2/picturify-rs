use crate::common::kernels::prewitt::PrewittKernels;
use crate::common::kernels::sobel::SobelKernels;
use crate::common::processors::CpuProcessor;
use crate::processors::noise::gaussian_blur::{
    GaussianBlurProcessor, GaussianBlurProcessorOptions,
};
use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::ProcessingPicturifyResult;
use picturify_core::geometry::coord::Coord;
use picturify_core::image::Rgba;
use picturify_core::pixel::colors::Colors;
use picturify_core::pixel::traits::RgbaF32Pixel;
use picturify_core::rayon::prelude::*;
use picturify_core::threading::progress::Progress;

pub enum CannyEdgeDetectionType {
    Sobel,
    Prewitt,
    Scharr,
}

pub struct CannyProcessorOptions {
    pub sigma: f32,
    pub radius: usize,
    pub edge_detection_type: CannyEdgeDetectionType,
    pub low_threshold: f32,
    pub high_threshold: f32,
}

pub struct CannyProcessor {
    options: CannyProcessorOptions,
}

impl CannyProcessor {
    pub fn new(options: CannyProcessorOptions) -> Self {
        Self { options }
    }
}

impl CpuProcessor for CannyProcessor {
    fn process(
        &self,
        image: FastImage,
        mut progress: Progress,
    ) -> ProcessingPicturifyResult<FastImage> {
        progress.setup(5);
        let blurred_image = self.apply_gaussian_blur(image)?;
        progress.increment();
        let gradient = self.apply_edge_detection(blurred_image)?;
        progress.increment();
        let non_max_suppression = self.apply_non_maximum_supression(gradient)?;
        progress.increment();
        let double_threshold = self.apply_double_threshold(non_max_suppression)?;
        progress.increment();
        let hysteresis = self.apply_hysteresis(double_threshold)?;
        progress.increment();
        Ok(hysteresis)
    }
}

#[derive(Debug, Clone, Copy)]
enum GradientDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

struct ImageGradientResult {
    gradient_magnitude: Vec<f32>,
    gradient_direction: Vec<GradientDirection>,
    width: usize,
    height: usize,
}

impl ImageGradientResult {
    fn new(width: usize, height: usize) -> Self {
        Self {
            gradient_magnitude: vec![0.0; width * height],
            gradient_direction: vec![GradientDirection::North; width * height],
            width,
            height,
        }
    }
}

struct IntensityResult {
    intensity: Vec<f32>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PixelThresholdValue {
    None,
    Low,
    High,
}

struct DoubleThresholdResult {
    pixels: Vec<PixelThresholdValue>,
    width: usize,
    height: usize,
}

impl CannyProcessor {
    fn apply_gaussian_blur(&self, image: FastImage) -> ProcessingPicturifyResult<FastImage> {
        let sigma = self.options.sigma;
        let radius = self.options.radius;

        let processor = GaussianBlurProcessor::new(GaussianBlurProcessorOptions {
            use_fast_approximation: false,
            sigma,
            radius,
        });

        processor.process(image, Progress::new())
    }

    fn apply_edge_detection(
        &self,
        image: FastImage,
    ) -> ProcessingPicturifyResult<ImageGradientResult> {
        let kernels = match self.options.edge_detection_type {
            CannyEdgeDetectionType::Sobel => SobelKernels::create(),
            CannyEdgeDetectionType::Prewitt => PrewittKernels::create(),
            CannyEdgeDetectionType::Scharr => {
                unimplemented!("Scharr kernels are not implemented yet")
            }
        }?;

        let (width_usize, height_usize): (usize, usize) = image.size().into();
        let (width_i32, height_i32) = (width_usize as i32, height_usize as i32);

        let kernel_radius = kernels.radius() as i32;
        let mut result = ImageGradientResult::new(width_usize, height_usize);

        result
            .gradient_magnitude
            .iter_mut()
            .zip(result.gradient_direction.iter_mut())
            .enumerate()
            .par_bridge()
            .for_each(|(index_1d, (gradient_magnitude, gradient_direction))| {
                let pixel_coord = Coord::from_1d_index(index_1d, width_usize);

                if pixel_coord.x() < kernel_radius
                    || pixel_coord.x() >= width_i32 - kernel_radius
                    || pixel_coord.y() < kernel_radius
                    || pixel_coord.y() >= height_i32 - kernel_radius
                {
                    return;
                }

                let mut magnitude_x = 0.0;
                let mut magnitude_y = 0.0;

                kernels.iter().for_each(|(coord, x_value, y_value)| {
                    let pixel = image.get_image_pixel((pixel_coord + coord) - kernel_radius);

                    let colors = pixel.red_f32() + pixel.green_f32() + pixel.blue_f32();

                    magnitude_x += x_value * colors;
                    magnitude_y += y_value * colors;
                });

                magnitude_x /= 3.0;
                magnitude_y /= 3.0;

                let actual_magnitude = (magnitude_x.powi(2) + magnitude_y.powi(2)).sqrt();
                *gradient_magnitude = actual_magnitude;

                let direction = match (magnitude_y / magnitude_x).atan() {
                    angle if angle < -3.0 * std::f32::consts::FRAC_PI_4 => GradientDirection::North,
                    angle if angle < -std::f32::consts::FRAC_PI_4 => GradientDirection::NorthEast,
                    angle if angle < std::f32::consts::FRAC_PI_4 => GradientDirection::East,
                    angle if angle < 3.0 * std::f32::consts::FRAC_PI_4 => {
                        GradientDirection::SouthEast
                    }
                    angle if angle < 5.0 * std::f32::consts::FRAC_PI_4 => GradientDirection::South,
                    angle if angle < 7.0 * std::f32::consts::FRAC_PI_4 => {
                        GradientDirection::SouthWest
                    }
                    angle if angle < 9.0 * std::f32::consts::FRAC_PI_4 => GradientDirection::West,
                    _ => GradientDirection::NorthWest,
                };

                *gradient_direction = direction;
            });

        Ok(result)
    }

    fn apply_non_maximum_supression(
        &self,
        gradient: ImageGradientResult,
    ) -> ProcessingPicturifyResult<IntensityResult> {
        let width = gradient.width;
        let height = gradient.height;

        let mut result = IntensityResult {
            intensity: gradient.gradient_magnitude.clone(),
            width,
            height,
        };

        result
            .intensity
            .iter_mut()
            .enumerate()
            .par_bridge()
            .for_each(|(index_1d, magnitude)| {
                let pixel_coord = Coord::from_1d_index(index_1d, width);

                if pixel_coord.x() == 0
                    || pixel_coord.x() == width as i32 - 1
                    || pixel_coord.y() == 0
                    || pixel_coord.y() == height as i32 - 1
                {
                    return;
                }

                let direction = gradient.gradient_direction[index_1d];

                let current_magnitude = gradient.gradient_magnitude[index_1d];

                match direction {
                    GradientDirection::North | GradientDirection::South => {
                        let north_magnitude = gradient.gradient_magnitude[index_1d - width];
                        let south_magnitude = gradient.gradient_magnitude[index_1d + width];

                        if current_magnitude < north_magnitude
                            || current_magnitude < south_magnitude
                        {
                            *magnitude = 0.0;
                        }
                    }
                    GradientDirection::NorthEast | GradientDirection::SouthWest => {
                        let north_east_magnitude =
                            gradient.gradient_magnitude[index_1d - width + 1];
                        let south_west_magnitude =
                            gradient.gradient_magnitude[index_1d + width - 1];

                        if current_magnitude < north_east_magnitude
                            || current_magnitude < south_west_magnitude
                        {
                            *magnitude = 0.0;
                        }
                    }
                    GradientDirection::East | GradientDirection::West => {
                        let east_magnitude = gradient.gradient_magnitude[index_1d + 1];
                        let west_magnitude = gradient.gradient_magnitude[index_1d - 1];

                        if current_magnitude < east_magnitude || current_magnitude < west_magnitude
                        {
                            *magnitude = 0.0;
                        }
                    }
                    GradientDirection::NorthWest | GradientDirection::SouthEast => {
                        let north_west_magnitude =
                            gradient.gradient_magnitude[index_1d - width - 1];
                        let south_east_magnitude =
                            gradient.gradient_magnitude[index_1d + width + 1];

                        if current_magnitude < north_west_magnitude
                            || current_magnitude < south_east_magnitude
                        {
                            *magnitude = 0.0;
                        }
                    }
                }
            });

        Ok(result)
    }

    fn apply_double_threshold(
        &self,
        gradient: IntensityResult,
    ) -> ProcessingPicturifyResult<DoubleThresholdResult> {
        let width = gradient.width;
        let height = gradient.height;

        let mut result = DoubleThresholdResult {
            pixels: vec![PixelThresholdValue::None; width * height],
            width,
            height,
        };

        result
            .pixels
            .iter_mut()
            .enumerate()
            .par_bridge()
            .for_each(|(index_1d, pixel)| {
                let magnitude = gradient.intensity[index_1d];

                if magnitude < self.options.low_threshold {
                    *pixel = PixelThresholdValue::Low;
                } else if magnitude > self.options.high_threshold {
                    *pixel = PixelThresholdValue::High;
                }
            });

        Ok(result)
    }

    fn apply_hysteresis(
        &self,
        threshold: DoubleThresholdResult,
    ) -> ProcessingPicturifyResult<FastImage> {
        let width = threshold.width;
        let height = threshold.height;
        let mut output_image = FastImage::empty((width, height).into());

        // Create a copy of the threshold result to modify in-place
        let input_pixels = threshold.pixels.clone();
        let mut processed_pixels = threshold.pixels.clone();
        let mut visited_pixels = vec![false; width * height];

        // Helper function to recursively follow edges
        fn follow_edges(
            x: isize,
            y: isize,
            width: usize,
            height: usize,
            input_pixels: &Vec<PixelThresholdValue>,
            processed_pixels: &mut Vec<PixelThresholdValue>,
            visited_pixels: &mut Vec<bool>,
        ) {
            if visited_pixels[(y * width as isize + x) as usize] {
                return;
            }

            let directions = [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ];

            let index = |x: isize, y: isize| (y * width as isize + x) as usize;

            for &(dx, dy) in directions.iter() {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                    let idx = index(nx, ny);
                    if input_pixels[idx] == PixelThresholdValue::Low {
                        processed_pixels[idx] = PixelThresholdValue::High;
                        visited_pixels[idx] = true;
                        follow_edges(
                            nx,
                            ny,
                            width,
                            height,
                            input_pixels,
                            processed_pixels,
                            visited_pixels,
                        );
                    }
                }
            }
        }

        // Perform edge tracking by hysteresis
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                if input_pixels[idx] == PixelThresholdValue::High {
                    follow_edges(
                        x as isize,
                        y as isize,
                        width,
                        height,
                        &input_pixels,
                        &mut processed_pixels,
                        &mut visited_pixels,
                    );
                }
            }
        }

        // Set the pixel values in the output image based on the processed pixels
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                match processed_pixels[idx] {
                    PixelThresholdValue::None => {
                        output_image.set_image_pixel((x, y).into(), Rgba::black())
                    }
                    PixelThresholdValue::Low => {
                        output_image.set_image_pixel((x, y).into(), Rgba::black())
                    }
                    PixelThresholdValue::High => {
                        output_image.set_image_pixel((x, y).into(), Rgba::white())
                    }
                }
            }
        }

        Ok(output_image)
    }
}
