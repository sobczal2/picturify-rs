use clap::ArgMatches;
use picturify_core::fast_image::fast_image::FastImage;
use picturify_pipeline::noise::kuwahara::{KuwaharaPipeline, KuwaharaPipelineOptions};
use picturify_pipeline::pipeline::Pipeline;

pub struct KuwaharaCommandHandler;

impl KuwaharaCommandHandler {
    pub fn handle(fast_image: FastImage, args: ArgMatches) -> FastImage {
        let radius = args.get_one::<usize>("radius").unwrap();
        let kuwahara_pipeline = KuwaharaPipeline::new(KuwaharaPipelineOptions {
            radius: *radius,
        });
        let processed_image = kuwahara_pipeline.run(fast_image);
        processed_image
    }
}