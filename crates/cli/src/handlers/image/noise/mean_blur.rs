use clap::ArgMatches;

use picturify_pipeline::noise::mean_blur::{MeanBlurPipeline, MeanBlurPipelineOptions};

use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::{CommandHandler, run_pipeline};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct MeanBlurCommandHandler;

impl CommandHandler for MeanBlurCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let radius = args.get_one::<usize>(ArgType::Radius.to_id()).unwrap();
        let fast = args.get_one::<bool>(ArgType::Fast.to_id()).unwrap();
        let pipeline = MeanBlurPipeline::new(MeanBlurPipelineOptions {
            radius: *radius,
            fast: *fast,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
