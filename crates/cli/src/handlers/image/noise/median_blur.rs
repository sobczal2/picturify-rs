use clap::ArgMatches;

use picturify_pipeline::noise::median_blur::{MedianBlurPipeline, MedianBlurPipelineOptions};

use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::image::noise::median_blur::MedianBlurRadiusArg;
use crate::error::{CliPicturifyResult, MapToCliPicturifyResult};
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct MedianBlurCommandHandler;

impl CommandHandler for MedianBlurCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(FastArg::id()).map_to_unknown_error()?;
        let radius = args
            .get_one::<usize>(MedianBlurRadiusArg::id())
            .map_to_unknown_error()?;

        let pipeline = MedianBlurPipeline::new(MedianBlurPipelineOptions {
            fast: *fast,
            radius: *radius,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
