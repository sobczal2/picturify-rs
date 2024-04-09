// use palette::Srgba;
// use crate::common::execution::{CpuOptions, ExecutionPlan, Processor};
// use picturify_core::error::PicturifyResult;
// use picturify_core::image::fast_image::FastImage;
//
// pub enum EdgeEnlargementStrategy {
//     Constant(Srgba),
//     Mirror,
// }
//
// pub struct EdgeEnlargementProcessor {
//     execution_plan: ExecutionPlan,
//     options: EdgeEnlargementProcessorOptions,
// }
//
// pub struct EdgeEnlargementProcessorOptions {
//     pub border: usize,
//     pub strategy: EdgeEnlargementStrategy,
// }
//
// impl EdgeEnlargementProcessor {
//     pub fn new(radius: usize) -> EdgeEnlargementProcessor {
//         EdgeEnlargementProcessor {
//             execution_plan: ExecutionPlan::Cpu(Default::default()),
//             options: EdgeEnlargementProcessorOptions {
//                 border: radius,
//                 strategy: EdgeEnlargementStrategy::Mirror,
//             },
//         }
//     }
//
//     pub fn change_options(&mut self, action: fn(&mut EdgeEnlargementProcessorOptions)) {
//         action(&mut self.options);
//     }
//
//     fn run_cpu(&self, fast_image: FastImage, cpu_options: CpuOptions) -> FastImage {
//         let new_width = fast_image.get_width() + self.options.border * 2;
//         let new_height = fast_image.get_height() + self.options.border * 2;
//
//         let mut new_image = FastImage::empty(new_width, new_height);
//
//         let width = fast_image.get_width();
//         let height = fast_image.get_height();
//
//         cpu_options.build_thread_pool().install(|| {
//             match self.options.strategy {
//                 EdgeEnlargementStrategy::Constant(pixel) => {
//                     new_image.iterate_par_rgba(|new_pixel, x, y| {
//                         if x < self.options.border
//                             || x >= new_width - self.options.border
//                             || y < self.options.border
//                             || y >= new_height - self.options.border
//                         {
//                             *new_pixel = pixel;
//                         } else {
//                             *new_pixel = fast_image
//                                 .get_rgba(x - self.options.border, y - self.options.border);
//                         }
//                     });
//                 }
//                 EdgeEnlargementStrategy::Mirror => {
//                     new_image.iterate_par_rgba(|new_pixel, x, y| {
//                         if x < self.options.border
//                             || x >= new_width - self.options.border
//                             || y < self.options.border
//                             || y >= new_height - self.options.border
//                         {
//                             let x = if x < self.options.border {
//                                 self.options.border - x - 1 // For the left border
//                             } else if x >= width + self.options.border {
//                                 2 * width + self.options.border - x - 1 // For the right self.options.borderorder
//                             } else {
//                                 x - self.options.border // For the middle section
//                             };
//
//                             let y = if y < self.options.border {
//                                 self.options.border - y - 1 // For the top self.options.borderorder
//                             } else if y >= height + self.options.border {
//                                 2 * height + self.options.border - y - 1 // For the self.options.borderottom self.options.borderorder
//                             } else {
//                                 y - self.options.border // For the middle section
//                             };
//
//                             *new_pixel = fast_image.get_rgba(
//                                 if x > new_width / 2 {
//                                     (new_width - x - 1) % width
//                                 } else {
//                                     x % width
//                                 },
//                                 if y > new_height / 2 {
//                                     (new_height - y - 1) % height
//                                 } else {
//                                     y % height
//                                 },
//                             );
//                         } else {
//                             *new_pixel = fast_image
//                                 .get_rgba(x - self.options.border, y - self.options.border);
//                         }
//                     });
//                 }
//             }
//         });
//
//         new_image
//     }
//
//     fn run_gpu(&self, _fast_image: FastImage) -> FastImage {
//         unimplemented!()
//     }
// }
//
// impl Processor for EdgeEnlargementProcessor {
//     fn set_execution_plan(&mut self, execution_plan: ExecutionPlan) -> PicturifyResult<()> {
//         self.execution_plan = execution_plan;
//         Ok(())
//     }
//
//     fn process(&self, fast_image: FastImage) -> FastImage {
//         match self.execution_plan {
//             ExecutionPlan::Cpu(options) => self.run_cpu(fast_image, options),
//             ExecutionPlan::Gpu => self.run_gpu(fast_image),
//         }
//     }
// }
