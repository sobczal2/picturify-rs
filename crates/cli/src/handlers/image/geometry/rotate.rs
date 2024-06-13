use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image_io::{read_image, write_image};
use crate::progress::pipeline_progress_bar::run_progress_bar_for_pipeline;
use clap::ArgMatches;
use picturify_core::geometry::angle::Angle;
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use picturify_pipeline::geometry::rotate::{RotatePipeline, RotatePipelineOptions};
use picturify_pipeline::pipeline::Pipeline;

pub struct RotateCommandHandler;

impl CommandHandler for RotateCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let angle = args.get_one::<Angle>(ArgType::Angle.to_id()).unwrap();
        let rotate_pipeline = RotatePipeline::new(RotatePipelineOptions { angle: *angle });

        let pipeline_progress = PipelineProgress::new();
        let pipeline_progress_clone = pipeline_progress.clone();

        let handle = std::thread::spawn(move || {
            run_progress_bar_for_pipeline(pipeline_progress_clone);
        });

        let result_image = rotate_pipeline.run(image, Some(pipeline_progress.clone()));

        handle.join().expect("Failed to join thread");

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
