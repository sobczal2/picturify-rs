use clap::ArgMatches;

use picturify_pipeline::color::negative::{NegativePipeline, NegativePipelineOptions};

use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::{CommandHandler, run_pipeline};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct NegativeCommandHandler;

impl CommandHandler for NegativeCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(ArgType::Fast.to_id()).unwrap();

        let pipeline = NegativePipeline::new(NegativePipelineOptions { fast: *fast });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
