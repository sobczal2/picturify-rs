pub struct LaplacianOfGaussianProcessorOptions {
    pub sigma: f32,
    pub radius: usize,
}

pub struct LaplacianOfGaussianProcessor {
    options: LaplacianOfGaussianProcessorOptions,
}

impl LaplacianOfGaussianProcessor {
    pub fn new(options: LaplacianOfGaussianProcessorOptions) -> Self {
        Self { options }
    }
}
