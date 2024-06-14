use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};
use clap::ArgMatches;
use picturify_core::geometry::size::Size;
use picturify_pipeline::geometry::scale::{ScalePipeline, ScalePipelineOptions};
use picturify_processing::processors::geometry::scale::ScaleStrategy;

pub struct ScaleCommandHandler;

impl CommandHandler for ScaleCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let size = args.get_one::<Size>(ArgType::Size.to_id()).unwrap();
        let strategy = args
            .get_one::<ScaleStrategy>(ArgType::ScaleStrategy.to_id())
            .unwrap();

        let pipeline = ScalePipeline::new(ScalePipelineOptions {
            size: *size,
            strategy: *strategy,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
