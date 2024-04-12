use std::time::Instant;
use picturify_core::fast_image::fast_image::FastImage;
use picturify_core::fast_image::io::{ReadFromFile, WriteToFile};
use picturify_movie::movie_pipe::MoviePipe;
use picturify_pipeline::color::negative::{NegativePipeline, NegativePipelineOptions};
use picturify_pipeline::color::sepia::{SepiaPipeline, SepiaPipelineOptions};
use picturify_pipeline::edge::sobel::{SobelPipeline, SobelPipelineOptions};
use picturify_pipeline::edge::sobel_rgb::{SobelRgbPipeline, SobelRgbPipelineOptions};
use picturify_pipeline::noise::kuwahara::{KuwaharaPipeline, KuwaharaPipelineOptions};
use picturify_pipeline::noise::median::{MedianPipeline, MedianPipelineOptions};
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::edge::sobel::{SobelProcessor, SobelProcessorOptions};
use picturify_processing::processors::edge::sobel_rgb::SobelRgbProcessor;

fn main() {
    // run_image();
    run_movie();
}


fn run_movie() {
    let movie_pipe = MoviePipe::new(
        "/home/sobczal/Downloads/2055.mp4".to_string(),
        "/home/sobczal/Downloads/2055_sobel.mp4".to_string(),
        Box::new(SobelRgbPipeline::new(
            SobelRgbPipelineOptions {
                use_fast_approximation: true,
            }
        )),
    );

    movie_pipe.process();
}

fn run_image() {
    let fast_image = FastImage::read_from_file("/home/sobczal/Downloads/ryan_mid_res.jpg").unwrap();
    let image = *fast_image;
    let start = Instant::now();



    let processor = SobelRgbProcessor::new();
    let image = processor.process(image);

    let duration = start.elapsed();
    println!("Time elapsed in processing fast_image is: {:?}", duration);

    image
        .write_to_file("/home/sobczal/Downloads/output.png")
        .unwrap();
}