use clap::ArgMatches;
use picturify_core::fast_image::FastImage;

pub struct MedianCommandHandler;

impl MedianCommandHandler {
    pub fn handle(fast_image: FastImage, args: ArgMatches) -> FastImage {
        let radius = args.get_one::<usize>("radius").unwrap();
        // let median_pipeline = MedianPipeline::new(MedianPipelineOptions { radius: *radius });
        
        // median_pipeline.run(fast_image)
        fast_image
    }
}
