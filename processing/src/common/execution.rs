use picturify_core::error::PicturifyResult;
use picturify_core::fast_image::FastImage;
use picturify_core::rayon::{current_num_threads, ThreadPool, ThreadPoolBuilder};

#[derive(Copy, Clone)]
#[derive(Default)]
pub struct CpuOptions {
    pub num_threads: Option<usize>,
}

impl CpuOptions {
    pub fn get_num_threads(&self) -> usize {
        self.num_threads.unwrap_or(current_num_threads())
    }

    pub fn build_thread_pool(&self) -> ThreadPool {
        ThreadPoolBuilder::new()
            .num_threads(self.get_num_threads())
            .build()
            .unwrap()
    }
}



#[derive(Copy, Clone)]
pub enum ExecutionPlan {
    Cpu(CpuOptions),
    Gpu,
}

pub trait Processor {
    fn set_execution_plan(&mut self, execution_plan: ExecutionPlan) -> PicturifyResult<()>;
    fn process(&self, fast_image: FastImage) -> FastImage;
}
