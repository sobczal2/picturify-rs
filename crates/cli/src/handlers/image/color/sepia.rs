#[cfg(feature = "gpu")]
use crate::commands::common::args::common::GpuArg;
use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::error::{CliPicturifyResult, MapToCliPicturifyResult};
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};
use clap::ArgMatches;
use picturify_pipeline::color::sepia::{SepiaPipeline, SepiaPipelineOptions};

pub struct SepiaCommandHandler;

impl CommandHandler for SepiaCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(FastArg::id()).map_to_unknown_error()?;

        #[cfg(feature = "gpu")]
        let gpu = args.get_one::<bool>(GpuArg::id()).map_to_unknown_error()?;

        let pipeline = SepiaPipeline::new(SepiaPipelineOptions {
            fast: *fast,
            #[cfg(feature = "gpu")]
            use_gpu: *gpu,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
