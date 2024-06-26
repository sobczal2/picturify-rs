use clap::ArgMatches;

use picturify_pipeline::noise::bilateral_blur::{
    BilateralBlurPipeline, BilateralBlurPipelineOptions,
};

use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::image::noise::bilateral_blur::{
    BilateralBlurIntensitySigmaArg, BilateralBlurRadiusArg, BilateralBlurSpatialSigmaArg,
};
use crate::error::{CliPicturifyResult, MapToCliPicturifyResult};
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};

pub struct BilateralBlurCommandHandler;

impl CommandHandler for BilateralBlurCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(FastArg::id()).map_to_unknown_error()?;
        let radius = args
            .get_one::<usize>(BilateralBlurRadiusArg::id())
            .map_to_unknown_error()?;
        let spatial_sigma = args
            .get_one::<f32>(BilateralBlurSpatialSigmaArg::id())
            .map_to_unknown_error()?;
        let intensity_sigma = args
            .get_one::<f32>(BilateralBlurIntensitySigmaArg::id())
            .map_to_unknown_error()?;

        let pipeline = BilateralBlurPipeline::new(BilateralBlurPipelineOptions {
            fast: *fast,
            radius: *radius,
            sigma_spatial: *spatial_sigma,
            sigma_intensity: *intensity_sigma,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
