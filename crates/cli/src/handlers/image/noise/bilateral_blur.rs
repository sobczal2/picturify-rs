use clap::ArgMatches;

use picturify_pipeline::noise::bilateral_blur::{
    BilateralBlurPipeline, BilateralBlurPipelineOptions,
};

use crate::commands::common::arg::ArgType;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct BilateralBlurCommandHandler;

impl CommandHandler for BilateralBlurCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(ArgType::Fast.to_id()).unwrap();
        let radius = args.get_one::<usize>(ArgType::Radius.to_id()).unwrap();
        let sigma_spatial = args.get_one::<f32>(ArgType::SigmaSpatial.to_id()).unwrap();
        let sigma_intensity = args
            .get_one::<f32>(ArgType::SigmaIntensity.to_id())
            .unwrap();

        let pipeline = BilateralBlurPipeline::new(BilateralBlurPipelineOptions {
            fast: *fast,
            radius: *radius,
            sigma_spatial: *sigma_spatial,
            sigma_intensity: *sigma_intensity,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
