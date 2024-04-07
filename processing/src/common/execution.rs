use picturify_core::image::fast_image::FastImage;

#[derive(Copy, Clone)]
pub struct MultiThreadCpuOptions {
    pub num_threads: Option<usize>,
}

impl MultiThreadCpuOptions {
    pub fn get_num_threads(&self) -> usize {
        self.num_threads.unwrap_or(rayon::current_num_threads())
    }

    pub fn build_thread_pool(&self) -> rayon::ThreadPool {
        rayon::ThreadPoolBuilder::new()
            .num_threads(self.get_num_threads())
            .build()
            .unwrap()
    }
}

impl Default for MultiThreadCpuOptions {
    fn default() -> Self {
        MultiThreadCpuOptions { num_threads: None }
    }
}

pub enum ExecutionPlan {
    SingleThreadCpu,
    MultiThreadCpu(MultiThreadCpuOptions),
    Gpu,
}
