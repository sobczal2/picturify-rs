use picturify_core::fast_image::FastImage;
use picturify_core::image::Rgba;
use picturify_core::palette::LinSrgba;

pub fn create_sobel_kernel_x() -> [[f32; 3]; 3] {
    [[-1.0, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]]
}

pub fn create_sobel_kernel_y() -> [[f32; 3]; 3] {
    [[-1.0, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]]
}

pub struct ConvolutionKernel {
    pub values: Vec<Vec<f32>>,
}

impl ConvolutionKernel {
    pub fn new(values: Vec<Vec<f32>>) -> Self {
        ConvolutionKernel { values }
    }

    pub fn new_mean(radius: usize) -> Self {
        let value = 1.0 / ((2 * radius + 1) * (2 * radius + 1)) as f32;
        let values = vec![vec![value; 2 * radius + 1]; 2 * radius + 1];
        ConvolutionKernel { values }
    }

    pub fn new_sharpen() -> Self {
        ConvolutionKernel {
            values: vec![
                vec![0.0, -1.0, 0.0],
                vec![-1.0, 5.0, -1.0],
                vec![0.0, -1.0, 0.0],
            ],
        }
    }

    pub fn new_gaussian(radius: usize, sigma: f32) -> Self {
        let mut values = vec![vec![0.0; 2 * radius + 1]; 2 * radius + 1];

        let sigma_squared = sigma * sigma;
        let two_sigma_squared = 2.0 * sigma_squared;

        let mut sum = 0.0;

        for i in 0..2 * radius + 1 {
            for j in 0..2 * radius + 1 {
                let x = i as f32 - radius as f32;
                let y = j as f32 - radius as f32;

                let value = (-(x * x + y * y) / two_sigma_squared).exp();
                values[i][j] = value;
                sum += value;
            }
        }

        for i in 0..2 * radius + 1 {
            for j in 0..2 * radius + 1 {
                values[i][j] /= sum;
            }
        }

        ConvolutionKernel { values }
    }

    #[inline(always)]
    pub fn get_width(&self) -> usize {
        self.values[0].len()
    }

    #[inline(always)]
    pub fn get_height(&self) -> usize {
        self.values.len()
    }

    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.values[y][x]
    }

    pub fn convolve_rgb_fast(&self, fast_image: &FastImage, x: usize, y: usize) -> Rgba<u8> {
        let mut result_red_f32 = 0f32;
        let mut result_green_f32 = 0f32;
        let mut result_blue_f32 = 0f32;

        for i in 0..self.get_width() {
            for j in 0..self.get_height() {
                let kernel_value = self.get(i, j);

                if kernel_value == 0.0 {
                    continue;
                }

                let image_pixel = fast_image
                    .get_image_pixel(x + i - self.get_width() / 2, y + j - self.get_height() / 2);
                result_red_f32 += image_pixel.0[0] as f32 * kernel_value;
                result_green_f32 += image_pixel.0[1] as f32 * kernel_value;
                result_blue_f32 += image_pixel.0[2] as f32 * kernel_value;
            }
        }

        let result_alpha = fast_image.get_image_pixel(x, y).0[3];

        Rgba([
            result_red_f32.clamp(0.0, 255.0) as u8,
            result_green_f32.clamp(0.0, 255.0) as u8,
            result_blue_f32.clamp(0.0, 255.0) as u8,
            result_alpha,
        ])
    }

    pub fn convolve_rgb_slow(&self, fast_image: &FastImage, x: usize, y: usize) -> LinSrgba {
        let mut result_red_f32 = 0f32;
        let mut result_green_f32 = 0f32;
        let mut result_blue_f32 = 0f32;

        for i in 0..self.get_width() {
            for j in 0..self.get_height() {
                let kernel_value = self.get(i, j);

                if kernel_value == 0.0 {
                    continue;
                }

                let image_pixel = fast_image.get_lin_srgba_pixel(
                    x + i - self.get_width() / 2,
                    y + j - self.get_height() / 2,
                );

                result_red_f32 += image_pixel.red * kernel_value;
                result_green_f32 += image_pixel.green * kernel_value;
                result_blue_f32 += image_pixel.blue * kernel_value;
            }
        }

        let result_alpha = fast_image.get_lin_srgba_pixel(x, y).alpha;

        LinSrgba::new(
            result_red_f32.clamp(0.0, 1.0),
            result_green_f32.clamp(0.0, 1.0),
            result_blue_f32.clamp(0.0, 1.0),
            result_alpha,
        )
    }
}
