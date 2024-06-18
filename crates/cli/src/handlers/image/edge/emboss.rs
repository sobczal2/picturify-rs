use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::error::{CliPicturifyResult, MapToCliPicturifyResult};
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};
use clap::ArgMatches;
use picturify_pipeline::edge::emboss::{EmbossPipeline, EmbossPipelineOptions};

pub struct EmbossCommandHandler;

impl CommandHandler for EmbossCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(FastArg::id()).map_to_unknown_error()?;

        let pipeline = EmbossPipeline::new(EmbossPipelineOptions { fast: *fast });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
