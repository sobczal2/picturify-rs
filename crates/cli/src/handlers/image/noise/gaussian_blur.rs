use clap::ArgMatches;

use picturify_pipeline::noise::gaussian_blur::{GaussianBlurPipeline, GaussianBlurPipelineOptions};

use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::image::noise::gaussian_blur::{GaussianBlurRadiusArg, GaussianBlurSigmaArg};
use crate::error::{CliPicturifyResult, MapToCliPicturifyResult};
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct GaussianBlurCommandHandler;

impl CommandHandler for GaussianBlurCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(FastArg::id()).map_to_unknown_error()?;
        let radius = args
            .get_one::<usize>(GaussianBlurRadiusArg::id())
            .map_to_unknown_error()?;
        let sigma = args
            .get_one::<f32>(GaussianBlurSigmaArg::id())
            .map_to_unknown_error()?;

        let pipeline = GaussianBlurPipeline::new(GaussianBlurPipelineOptions {
            fast: *fast,
            radius: *radius,
            sigma: *sigma,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
