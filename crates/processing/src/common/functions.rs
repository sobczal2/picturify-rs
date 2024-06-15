use std::f32::consts::PI;

pub fn gaussian_2d(x: f32, y: f32, two_sigma_squared: f32) -> f32 {
    let exponent = -((x * x + y * y) / two_sigma_squared);
    (1.0 / (PI * two_sigma_squared)) * exponent.exp()
}

pub fn gaussian_1d(x: f32, sigma: f32) -> f32 {
    let two_sigma_squared = 2.0 * sigma * sigma;
    let exponent = -(x * x / two_sigma_squared);
    (1.0 / (PI * two_sigma_squared)).exp() * exponent.exp()
}
