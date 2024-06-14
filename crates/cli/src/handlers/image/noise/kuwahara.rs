use clap::ArgMatches;

use picturify_pipeline::noise::kuwahara::{KuwaharaPipeline, KuwaharaPipelineOptions};

use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::{CommandHandler, run_pipeline};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct KuwaharaCommandHandler;

impl CommandHandler for KuwaharaCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(ArgType::Fast.to_id()).unwrap();
        let radius = args.get_one::<usize>(ArgType::Radius.to_id()).unwrap();

        let pipeline = KuwaharaPipeline::new(KuwaharaPipelineOptions {
            fast: *fast,
            radius: *radius,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
