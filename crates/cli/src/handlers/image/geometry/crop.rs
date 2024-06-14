use clap::ArgMatches;

use picturify_pipeline::geometry::crop::{CropPipeline, CropPipelineOptions};
use picturify_processing::processors::geometry::crop::CropBorder;

use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::{CommandHandler, run_pipeline};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct CropCommandHandler;

impl CommandHandler for CropCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let crop_border = args.get_one::<CropBorder>(ArgType::Border.to_id()).unwrap();

        let pipeline = CropPipeline::new(CropPipelineOptions {
            crop_border: *crop_border,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
