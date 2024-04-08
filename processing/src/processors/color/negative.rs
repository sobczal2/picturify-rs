use crate::common::channel::ChannelSelector;
use crate::common::execution::{CpuOptions, ExecutionPlan};
use crate::common::process::{LayerPipe, LayerPipeRunner, Processor};
use picturify_core::error::PicturifyResult;
use picturify_core::image::fast_image::FastImage;

pub struct NegativeProcessor {
    channel_selector: ChannelSelector,
    execution_plan: ExecutionPlan,
}

impl NegativeProcessor {
    pub fn new() -> NegativeProcessor {
        NegativeProcessor {
            channel_selector: ChannelSelector::Rgba(Default::default()),
            execution_plan: ExecutionPlan::Cpu(Default::default()),
        }
    }

    fn run_cpu(&self, fast_image: FastImage, cpu_options: CpuOptions) -> FastImage {
        let layer_pipe = LayerPipe::new();
        let layer_pipe = layer_pipe.prepare_layers(&fast_image, self.channel_selector);
        let mut layer_pipe_runner = LayerPipeRunner::new(layer_pipe);

        cpu_options.build_thread_pool().install(|| {
            layer_pipe_runner.par_run_all_layers_if_enabled(
                |r, _x, _y| 255 - r,
                |g, _x, _y| 255 - g,
                |b, _x, _y| 255 - b,
                |a, _x, _y| 255 - a,
                |h, _x, _y| 360.0 - h,
                |s, _x, _y| 1.0 - s,
                |v, _x, _y| 1.0 - v,
                |l, _x, _y| 1.0 - l,
            );
        });

        layer_pipe_runner.get_final_image()
    }

    fn run_gpu(&self, _fast_image: FastImage) -> FastImage {
        unimplemented!()
    }
}

impl Processor for NegativeProcessor {
    fn set_execution_plan(&mut self, execution_plan: ExecutionPlan) -> PicturifyResult<()> {
        self.execution_plan = execution_plan;
        Ok(())
    }

    fn set_channel_selector(&mut self, channel_selector: ChannelSelector) -> PicturifyResult<()> {
        self.channel_selector = channel_selector;
        Ok(())
    }

    fn process(&self, fast_image: FastImage) -> FastImage {
        match self.execution_plan {
            ExecutionPlan::Cpu(options) => {
                return self.run_cpu(fast_image, options);
            }
            ExecutionPlan::Gpu => {
                return self.run_gpu(fast_image);
            }
        }
    }
}
