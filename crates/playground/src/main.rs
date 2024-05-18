use std::sync::{Arc, RwLock};
use std::time::Instant;

use picturify_core::fast_image::FastImage;
use picturify_core::fast_image::io::{ReadFromFile, WriteToFile};
use picturify_core::threading::progress::Progress;
use picturify_processing::common::execution::{Processor, WithOptions};
use picturify_processing::processors::color::grayscale::{GrayscaleProcessor, GrayscaleProcessorOptions, GrayscaleStrategy};
use picturify_processing::processors::geometry::crop::{CropProcessor, CropProcessorOptions};

fn main() {
    run_image();
    // run_movie();
}

#[allow(dead_code)]
fn run_movie() {
    // let custom_pipeline = CustomPipeline::new(|fast_image: FastImage| {
    //     let image = fast_image;
    //     let processor = MedianProcessor::with_options(MedianProcessorOptions { radius: 3 });
    //     let image = processor.process(image);
    //
    //     let processor = SobelRgbProcessor::new();
    //     let image = processor.process(image);
    //
    //     let processor = RemappingProcessor::with_options(RemappingProcessorOptions {
    //         function: RemappingFunction::Logarithmic { factor: 1.025 },
    //     });
    //     let image = processor.process(image);
    //
    //     let processor = ThresholdProcessor::with_options(ThresholdProcessorOptions {
    //         red_threshold: 128,
    //         green_threshold: 128,
    //         blue_threshold: 128,
    //     });
    //
    //     processor.process(image)
    // });
    //
    // let movie_pipe = MoviePipe::new(
    //     "/home/sobczal/Downloads/SampleVideo_1280x720_10mb.mp4".to_string(),
    //     "/home/sobczal/Downloads/SampleVideo_1280x720_10mb_sobel.mp4".to_string(),
    //     // Box::new(SobelRgbPipeline::with_options(SobelRgbPipelineOptions {
    //     //     use_fast_approximation: true,
    //     // })),
    //     Box::new(custom_pipeline),
    // );
    //
    // movie_pipe.process();
}

fn run_image() {
    let fast_image = *FastImage::read_from_file("/home/sobczal/Downloads/large.jpg").unwrap();

    let processor = CropProcessor::new().with_options(CropProcessorOptions {
        x: 100,
        y: 100,
        width: 2000,
        height: 2000,
    });
    
    let start = Instant::now();
    let fast_image = processor.process(fast_image, Arc::new(RwLock::new(Progress::new())));
    let duration = start.elapsed();
    println!("Time elapsed in grayscale is: {:?}", duration);
    fast_image
        .write_to_file("/home/sobczal/Downloads/output.png")
        .unwrap();
}
