use crate::commands::common::args::common::PicturifyArg;
use crate::commands::image::geometry::scale::{ScaleSizeArg, ScaleStrategyArg};
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
        let size = args.get_one::<Size>(ScaleSizeArg::id()).unwrap();
        let strategy = args
            .get_one::<ScaleStrategy>(ScaleStrategyArg::id())
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
