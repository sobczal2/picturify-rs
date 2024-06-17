use clap::ArgMatches;

use picturify_pipeline::color::grayscale::{GrayscalePipeline, GrayscalePipelineOptions};
use picturify_processing::processors::color::grayscale::GrayscaleStrategy;

use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::image::color::grayscale::GrayscaleStrategyArg;
use crate::error::{CliPicturifyResult, MapToCliPicturifyResult};
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct GrayscaleCommandHandler;

impl CommandHandler for GrayscaleCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let strategy = args
            .get_one::<GrayscaleStrategy>(GrayscaleStrategyArg::id())
            .map_to_unknown_error()?;
        let fast = args.get_one::<bool>(FastArg::id()).map_to_unknown_error()?;
        let pipeline = GrayscalePipeline::new(GrayscalePipelineOptions {
            strategy: *strategy,
            fast: *fast,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
