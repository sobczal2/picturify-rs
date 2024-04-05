use image::DynamicImage;

pub trait Processor {
    fn process(&self, image: DynamicImage, processor_runner: &ProcessorRunner) -> DynamicImage;
}

pub enum ProcessorRunner {
    RayonCpu(RayonCpuOptions),
    CudaGpu,
}

pub struct RayonCpuOptions {
    pub threads: usize,
}

impl Default for RayonCpuOptions {
    fn default() -> Self {
        RayonCpuOptions {
            threads: rayon::current_num_threads(),
        }
    }
}