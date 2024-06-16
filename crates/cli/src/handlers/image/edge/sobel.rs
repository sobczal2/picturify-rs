use clap::ArgMatches;

use picturify_pipeline::edge::sobel::{SobelPipeline, SobelPipelineOptions};

use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::image::edge::sobel::SobelRgbArg;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct SobelCommandHandler;

impl CommandHandler for SobelCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(FastArg::id()).unwrap();
        let rgb = args.get_one::<bool>(SobelRgbArg::id()).unwrap();

        let pipeline = SobelPipeline::new(SobelPipelineOptions {
            fast: *fast,
            rgb: *rgb,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
