use picturify_core::error::PicturifyResult;
use picturify_core::fast_image::FastImage;
use crate::common::execution::{CpuOptions, ExecutionPlan, Processor};
use crate::processors::internal::convolution::{ConvolutionProcessor, ConvolutionProcessorOptions};

pub struct MeanProcessorOptions {
    pub radius: usize,
    pub use_fast_approximation: bool,
}

impl Default for MeanProcessorOptions {
    fn default() -> MeanProcessorOptions {
        MeanProcessorOptions {
            radius: 3,
            use_fast_approximation: true,
        }
    }
}

pub struct MeanProcessor {
    execution_plan: ExecutionPlan,
    options: MeanProcessorOptions,
}

impl Default for MeanProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl MeanProcessor {
    pub fn new() -> MeanProcessor {
        MeanProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options: Default::default(),
        }
    }

    pub fn with_options(options: MeanProcessorOptions) -> MeanProcessor {
        MeanProcessor {
            execution_plan: ExecutionPlan::Cpu(Default::default()),
            options,
        }
    }

    fn run_cpu(&self, mut fast_image: FastImage, cpu_options: CpuOptions) -> FastImage {
        let radius = self.options.radius;

        let kernel = vec![1.0; (2 * radius + 1) * (2 * radius + 1)];
        let mut processor = ConvolutionProcessor::with_options(
            ConvolutionProcessorOptions {
                kernel,
                kernel_width: 2 * radius + 1,
                kernel_height: 2 * radius + 1,
                kernel_divisor: (2 * radius + 1) as f32 * (2 * radius + 1) as f32,
                kernel_offset: 0.0,
                use_fast_approximation: self.options.use_fast_approximation,
            },
        );
        processor.set_execution_plan(ExecutionPlan::Cpu(cpu_options)).unwrap();

        processor.process(fast_image)
    }

    fn run_gpu(&self, fast_image: FastImage) -> FastImage {
        unimplemented!()
    }
}

impl Processor for MeanProcessor {
    fn set_execution_plan(&mut self, execution_plan: ExecutionPlan) -> PicturifyResult<()> {
        self.execution_plan = execution_plan;
        Ok(())
    }

    fn process(&self, fast_image: FastImage) -> FastImage {
        match self.execution_plan {
            ExecutionPlan::Cpu(cpu_options) => self.run_cpu(fast_image, cpu_options),
            ExecutionPlan::Gpu => self.run_gpu(fast_image),
        }
    }
}
