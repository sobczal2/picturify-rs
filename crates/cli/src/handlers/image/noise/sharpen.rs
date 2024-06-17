use clap::ArgMatches;

use picturify_pipeline::noise::sharpen::{SharpenPipeline, SharpenPipelineOptions};

use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::error::{CliPicturifyResult, MapToCliPicturifyResult};
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct SharpenCommandHandler;

impl CommandHandler for SharpenCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(FastArg::id()).map_to_unknown_error()?;

        let pipeline = SharpenPipeline::new(SharpenPipelineOptions { fast: *fast });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
