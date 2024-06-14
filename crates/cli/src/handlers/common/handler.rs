use std::thread::{spawn, JoinHandle};

use clap::ArgMatches;

use picturify_core::core::fast_image::FastImage;
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use picturify_pipeline::pipeline::Pipeline;

use crate::error::{CliPicturifyError, CliPicturifyResult};
use crate::progress::pipeline_progress_bar::run_progress_bar_for_pipeline;

pub fn run_pipeline(
    image: FastImage,
    pipeline: Box<dyn Pipeline>,
) -> CliPicturifyResult<FastImage> {
    let bar = PipelineProgressBar::start();

    let result_image = pipeline.run(image, Some(bar.get_pipeline_progress()));

    bar.stop()?;

    Ok(result_image)
}

pub struct PipelineProgressBar {
    pipeline_progress: PipelineProgress,
    join_handle: JoinHandle<()>,
}

impl PipelineProgressBar {
    pub fn start() -> Self {
        let pipeline_progress = PipelineProgress::new();
        let pipeline_progress_clone = pipeline_progress.clone();

        let join_handle = spawn(move || {
            run_progress_bar_for_pipeline(pipeline_progress_clone);
        });

        PipelineProgressBar {
            pipeline_progress,
            join_handle,
        }
    }

    pub fn get_pipeline_progress(&self) -> PipelineProgress {
        self.pipeline_progress.clone()
    }

    pub fn stop(self) -> CliPicturifyResult<()> {
        self.join_handle
            .join()
            .map_err(|_| CliPicturifyError::ThreadingError)
    }
}

pub trait CommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()>;
}
