use clap::ArgMatches;

use picturify_core::geometry::angle::Angle;
use picturify_pipeline::geometry::rotate::{RotatePipeline, RotatePipelineOptions};

use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::{CommandHandler, run_pipeline};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct RotateCommandHandler;

impl CommandHandler for RotateCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let angle = args.get_one::<Angle>(ArgType::Angle.to_id()).unwrap();

        let pipeline = RotatePipeline::new(RotatePipelineOptions { angle: *angle });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
