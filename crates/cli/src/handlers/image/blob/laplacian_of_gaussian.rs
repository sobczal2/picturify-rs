use crate::commands::common::args::common::{FastArg, PicturifyArg};
use crate::commands::image::blob::laplacian_of_gaussian::{
    LaplacianOfGaussianRadiusArg, LaplacianOfGaussianSigmaArg,
};
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::{run_pipeline, CommandHandler};
use crate::handlers::common::image_io::{read_image, write_image};
use clap::ArgMatches;
use picturify_pipeline::blob::laplacian_of_gaussian::{
    LaplacianOfGaussianPipeline, LaplacianOfGaussianPipelineOptions,
};

pub struct LaplacianOfGaussianCommandHandler;

impl CommandHandler for LaplacianOfGaussianCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;
        let fast = args.get_one::<bool>(FastArg::id()).unwrap();
        let radius = args
            .get_one::<usize>(LaplacianOfGaussianRadiusArg::id())
            .unwrap();
        let sigma = args
            .get_one::<f32>(LaplacianOfGaussianSigmaArg::id())
            .unwrap();

        let pipeline = LaplacianOfGaussianPipeline::new(LaplacianOfGaussianPipelineOptions {
            fast: *fast,
            radius: *radius,
            sigma: *sigma,
        });

        let result_image = run_pipeline(image, Box::new(pipeline))?;

        write_image(result_image, args.clone())?;

        Ok(())
    }
}
