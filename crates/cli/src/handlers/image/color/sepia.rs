use clap::ArgMatches;
#[cfg(feature = "gpu")]
use log::warn;

use picturify_pipeline::color::sepia::{SepiaPipeline, SepiaPipelineOptions};

use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct SepiaCommandHandler;

impl CommandHandler for SepiaCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(ArgType::Fast.to_id()).unwrap();

        #[cfg(feature = "gpu")]
        let gpu = args.get_one::<bool>(ArgType::Gpu.to_id()).unwrap();

        #[cfg(feature = "gpu")]
        if *gpu && *fast {
            warn!("Fast flag is ignored when using GPU");
        }

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
