use picturify_core::image::fast_image::FastImage;

#[derive(Copy, Clone)]
pub struct CpuOptions {
    pub num_threads: Option<usize>,
}

impl CpuOptions {
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

impl Default for CpuOptions {
    fn default() -> Self {
        CpuOptions { num_threads: None }
    }
}

#[derive(Copy, Clone)]
pub enum ExecutionPlan {
    Cpu(CpuOptions),
    Gpu,
}
