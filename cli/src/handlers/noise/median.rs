use clap::ArgMatches;
use picturify_core::fast_image::fast_image::FastImage;
use picturify_pipeline::noise::median::{MedianPipeline, MedianPipelineOptions};
use picturify_pipeline::pipeline::Pipeline;

pub struct MedianCommandHandler;

impl MedianCommandHandler {
    pub fn handle(fast_image: FastImage, args: ArgMatches) -> FastImage {
        let radius = args.get_one::<usize>("radius").unwrap();
        let median_pipeline = MedianPipeline::new(MedianPipelineOptions {
            radius: *radius,
        });
        let processed_image = median_pipeline.run(fast_image);
        processed_image
    }
}