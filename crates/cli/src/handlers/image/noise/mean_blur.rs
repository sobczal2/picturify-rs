use clap::ArgMatches;

use picturify_pipeline::noise::mean_blur::{MeanBlurPipeline, MeanBlurPipelineOptions};

use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::image::noise::mean_blur::MeanBlurRadiusArg;
use crate::error::{CliPicturifyResult, MapToCliPicturifyResult};
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct MeanBlurCommandHandler;

impl CommandHandler for MeanBlurCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let radius = args
            .get_one::<usize>(MeanBlurRadiusArg::id())
            .map_to_unknown_error()?;
        let fast = args.get_one::<bool>(FastArg::id()).map_to_unknown_error()?;
        let pipeline = MeanBlurPipeline::new(MeanBlurPipelineOptions {
            radius: *radius,
            fast: *fast,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
