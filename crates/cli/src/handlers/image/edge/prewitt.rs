use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::image::edge::prewitt::PrewittRgbArg;
use crate::error::{CliPicturifyResult, MapToCliPicturifyResult};
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};
use picturify_pipeline::edge::prewitt::{PrewittPipeline, PrewittPipelineOptions};

pub struct PrewittCommandHandler;

impl CommandHandler for PrewittCommandHandler {
    fn handle(&self, args: clap::ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(FastArg::id()).map_to_unknown_error()?;
        let rgb = args
            .get_one::<bool>(PrewittRgbArg::id())
            .map_to_unknown_error()?;

        let pipeline = PrewittPipeline::new(PrewittPipelineOptions {
            fast: *fast,
            rgb: *rgb,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
