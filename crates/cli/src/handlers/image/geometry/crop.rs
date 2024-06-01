use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image_io::{read_image, write_image};
use crate::progress::pipeline_progress_bar::run_progress_bar_for_pipeline;
use clap::ArgMatches;
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use picturify_pipeline::geometry::crop::{CropPipeline, CropPipelineOptions};
use picturify_pipeline::pipeline::Pipeline;
use picturify_processing::processors::geometry::crop::CropBorder;

pub struct CropCommandHandler;

impl CommandHandler for CropCommandHandler {
    fn handle(args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let crop_border = args
            .get_one::<CropBorder>(ArgType::CropBorder.to_id())
            .unwrap();
        let crop_pipeline = CropPipeline::new(CropPipelineOptions {
            crop_border: crop_border.clone(),
        });

        let pipeline_progress = PipelineProgress::new();
        let pipeline_progress_clone = pipeline_progress.clone();

        let handle = std::thread::spawn(move || {
            run_progress_bar_for_pipeline(pipeline_progress_clone);
        });

        let result_image = crop_pipeline.run(image, Some(pipeline_progress.clone()));

        handle.join().expect("Failed to join thread");

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
