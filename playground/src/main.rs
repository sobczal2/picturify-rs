use std::time::Instant;
use picturify_core::fast_image::fast_image::FastImage;
use picturify_core::fast_image::io::{ReadFromFile, WriteToFile};
use picturify_core::palette::LinSrgba;
use picturify_movie::movie_pipe::MoviePipe;
use picturify_pipeline::color::sepia::SepiaPipeline;
use picturify_pipeline::custom::CustomPipeline;
use picturify_pipeline::edge::sobel_rgb::{SobelRgbPipeline, SobelRgbPipelineOptions};
use picturify_pipeline::pipeline::Pipeline;
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::color::remapping::{RemappingFunction, RemappingProcessor, RemappingProcessorOptions};
use picturify_processing::processors::color::threshold::{ThresholdProcessor, ThresholdProcessorOptions};
use picturify_processing::processors::edge::sobel_rgb::SobelRgbProcessor;
use picturify_processing::processors::noise::median::{MedianProcessor, MedianProcessorOptions};

fn main() {
    run_image();
    // run_movie();
}


fn run_movie() {

    // let custom_pipeline = CustomPipeline::new(
    //     |fast_image: FastImage| {
    //         let image = fast_image;
    //         let processor = MedianProcessor::with_options(MedianProcessorOptions {
    //             radius: 3,
    //         });
    //         let image = processor.process(image);
    // 
    //         let processor = SobelRgbProcessor::new();
    //         let image = processor.process(image);
    // 
    //         let processor = RemappingProcessor::with_options(RemappingProcessorOptions {
    //             function: RemappingFunction::Logarithmic { factor: 1.025 },
    //         });
    //         let image = processor.process(image);
    // 
    //         let processor = ThresholdProcessor::with_options(ThresholdProcessorOptions {
    //             red_threshold: 128,
    //             green_threshold: 128,
    //             blue_threshold: 128,
    //         });
    //         let image = processor.process(image);
    // 
    //         image
    //     }
    // );
    
    let movie_pipe = MoviePipe::new(
        "/home/sobczal/Downloads/2055.mp4".to_string(),
        "/home/sobczal/Downloads/2055_sobel.mp4".to_string(),
        // Box::new(SobelRgbPipeline::new(
        //     SobelRgbPipelineOptions {
        //         use_fast_approximation: true,
        //     }
        // )),
        Box::new(SobelRgbPipeline::with_options(
            SobelRgbPipelineOptions {
                use_fast_approximation: true,
            }
        )),
    );

    movie_pipe.process();
}

fn run_image() {
    let fast_image = FastImage::read_from_file("/home/sobczal/Downloads/prada.jpg").unwrap();
    let image = *fast_image;
    let start = Instant::now();

    let custom_pipeline = CustomPipeline::new(
        |fast_image: FastImage| {
            let image = fast_image;
            let processor = MedianProcessor::with_options(MedianProcessorOptions {
                radius: 3,
            });
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
            let image = processor.process(image);

            image
        }
    );
    
    let image = custom_pipeline.run(image);

    let duration = start.elapsed();
    println!("Time elapsed in processing fast_image is: {:?}", duration);

    image
        .write_to_file("/home/sobczal/Downloads/output.png")
        .unwrap();
}