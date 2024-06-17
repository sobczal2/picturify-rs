use clap::ArgMatches;

use picturify_pipeline::color::quantization::{QuantizationPipeline, QuantizationPipelineOptions};

use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::image::color::quantization::QuantizationLevelsArg;
use crate::error::{CliPicturifyResult, MapToCliPicturifyResult};
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct QuantizationCommandHandler;

impl CommandHandler for QuantizationCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(FastArg::id()).map_to_unknown_error()?;
        let levels = args
            .get_one::<u8>(QuantizationLevelsArg::id())
            .map_to_unknown_error()?;

        let pipeline = QuantizationPipeline::new(QuantizationPipelineOptions {
            fast: *fast,
            levels: *levels,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
