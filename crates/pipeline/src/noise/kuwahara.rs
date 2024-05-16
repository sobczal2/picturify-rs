// use crate::pipeline::Pipeline;
// use picturify_core::fast_image::FastImage;
// use picturify_core::palette::Srgba;
// use picturify_processing::common::execution::Processor;
// use picturify_processing::processors::geometry::crop::CropProcessor;
// use picturify_processing::processors::geometry::enlargement::{
//     EnlargementProcessor, EnlargementProcessorOptions, EnlargementStrategy,
// };
// use picturify_processing::processors::noise::kuwahara::{
//     KuwaharaProcessor, KuwaharaProcessorOptions,
// };
// 
// pub struct KuwaharaPipelineOptions {
//     pub radius: usize,
// }
// 
// pub struct KuwaharaPipeline {
//     options: KuwaharaPipelineOptions,
// }
// 
// impl KuwaharaPipeline {
//     pub fn new(kuwahara_pipeline_options: KuwaharaPipelineOptions) -> Self {
//         Self {
//             options: kuwahara_pipeline_options,
//         }
//     }
// }
// 
// impl Pipeline for KuwaharaPipeline {
//     fn run(&self, fast_image: FastImage) -> FastImage {
//         let radius = self.options.radius;
//         let original_width = fast_image.get_width();
//         let original_height = fast_image.get_height();
// 
//         let enlargement_processor =
//             EnlargementProcessor::with_options(EnlargementProcessorOptions {
//                 border: radius,
//                 strategy: EnlargementStrategy::Constant(Srgba::new(0.0, 0.0, 0.0, 1.0)),
//             });
// 
//         let enlarged_image = enlargement_processor.process(fast_image);
// 
//         let kuwahara_processor =
//             KuwaharaProcessor::with_options(KuwaharaProcessorOptions { radius });
// 
//         let kuwahara_image = kuwahara_processor.process(enlarged_image);
// 
//         let crop_processor = CropProcessor::new(radius, radius, original_width, original_height);
// 
//         crop_processor.process(kuwahara_image)
//     }
// }
