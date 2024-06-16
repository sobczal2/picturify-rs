use clap::ArgMatches;

use picturify_pipeline::noise::kuwahara::{KuwaharaPipeline, KuwaharaPipelineOptions};

use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::image::noise::kuwahara::KuwaharaRadiusArg;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct KuwaharaCommandHandler;

impl CommandHandler for KuwaharaCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(FastArg::id()).unwrap();
        let radius = args.get_one::<usize>(KuwaharaRadiusArg::id()).unwrap();

        let pipeline = KuwaharaPipeline::new(KuwaharaPipelineOptions {
            fast: *fast,
            radius: *radius,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
