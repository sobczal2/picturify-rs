use clap::ArgMatches;

use picturify_pipeline::edge::sobel::{SobelPipeline, SobelPipelineOptions};

use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct SobelCommandHandler;

impl CommandHandler for SobelCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(ArgType::Fast.to_id()).unwrap();
        let rgb = args.get_one::<bool>(ArgType::Rgb.to_id()).unwrap();

        let pipeline = SobelPipeline::new(SobelPipelineOptions {
            fast: *fast,
            rgb: *rgb,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
