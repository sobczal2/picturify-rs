use clap::ArgMatches;

use picturify_pipeline::color::grayscale::{GrayscalePipeline, GrayscalePipelineOptions};
use picturify_processing::processors::color::grayscale::GrayscaleStrategy;

use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::{CommandHandler, run_pipeline};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct GrayscaleCommandHandler;

impl CommandHandler for GrayscaleCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let strategy = args
            .get_one::<GrayscaleStrategy>(ArgType::GrayscaleStrategy.to_id())
            .unwrap();
        let fast = args.get_one::<bool>(ArgType::Fast.to_id()).unwrap();
        let pipeline = GrayscalePipeline::new(GrayscalePipelineOptions {
            strategy: *strategy,
            fast: *fast,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
