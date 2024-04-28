use picturify_core::fast_image::FastImage;
use picturify_core::fast_image::io::{ReadFromFile, WriteToFile};

use picturify_movie::movie_pipe::MoviePipe;

use picturify_pipeline::custom::CustomPipeline;
use picturify_pipeline::edge::sobel_rgb::{SobelRgbPipeline, SobelRgbPipelineOptions};
use picturify_pipeline::pipeline::Pipeline;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::color::remapping::{
    RemappingFunction, RemappingProcessor, RemappingProcessorOptions,
};
use picturify_processing::processors::color::threshold::{
    ThresholdProcessor, ThresholdProcessorOptions,
};
use picturify_processing::processors::edge::sobel_rgb::SobelRgbProcessor;
use picturify_processing::processors::noise::median::{MedianProcessor, MedianProcessorOptions};
use std::time::Instant;
use picturify_processing::processors::noise::mean::{MeanProcessor, MeanProcessorOptions};

fn main() {
    run_image();
    // run_movie();
}

#[allow(dead_code)]
fn run_movie() {
    let custom_pipeline = CustomPipeline::new(|fast_image: FastImage| {
        let image = fast_image;
        let processor = MedianProcessor::with_options(MedianProcessorOptions { radius: 3 });
        let image = processor.process(image);

        let processor = SobelRgbProcessor::new();
        let image = processor.process(image);

        let processor = RemappingProcessor::with_options(RemappingProcessorOptions {
            function: RemappingFunction::Logarithmic { factor: 1.025 },
        });
        let image = processor.process(image);

        let processor = ThresholdProcessor::with_options(ThresholdProcessorOptions {
            red_threshold: 128,
            green_threshold: 128,
            blue_threshold: 128,
        });


        processor.process(image)
    });

    let movie_pipe = MoviePipe::new(
        "/home/sobczal/Downloads/SampleVideo_1280x720_10mb.mp4".to_string(),
        "/home/sobczal/Downloads/SampleVideo_1280x720_10mb_sobel.mp4".to_string(),
        // Box::new(SobelRgbPipeline::with_options(SobelRgbPipelineOptions {
        //     use_fast_approximation: true,
        // })),
        Box::new(custom_pipeline),
    );

    movie_pipe.process();
}

fn run_image() {
    let fast_image = *FastImage::read_from_file("/home/sobczal/Downloads/ryan.jpg").unwrap();

    let processor = MeanProcessor::with_options(MeanProcessorOptions { radius: 10, use_fast_approximation: true });

    let start = Instant::now();
    let fast_image = processor.process(fast_image);
    println!("Elapsed: {:?}", start.elapsed());

    fast_image.write_to_file("/home/sobczal/Downloads/ryan_mean.png").unwrap();
}
