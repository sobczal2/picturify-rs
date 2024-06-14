use clap::ArgMatches;

use picturify_pipeline::color::brightness::{BrightnessPipeline, BrightnessPipelineOptions};

use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct BrightnessCommandHandler;

impl CommandHandler for BrightnessCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let factor = args.get_one::<f32>(ArgType::Factor.to_id()).unwrap();
        let pipeline = BrightnessPipeline::new(BrightnessPipelineOptions { factor: *factor });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
