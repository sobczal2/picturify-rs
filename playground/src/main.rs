
use picturify_movie::movie_pipe::MoviePipe;
use picturify_pipeline::color::negative::{NegativePipeline, NegativePipelineOptions};
use picturify_pipeline::color::sepia::{SepiaPipeline, SepiaPipelineOptions};
use picturify_pipeline::noise::kuwahara::{KuwaharaPipeline, KuwaharaPipelineOptions};
use picturify_pipeline::noise::median::{MedianPipeline, MedianPipelineOptions};

fn main() {
//     let fast_image = FastImage::read_from_file("/home/sobczal/Downloads/ryan_mid_res.jpg").unwrap();
//     let image = *fast_image;
//     let start = Instant::now();

    let movie_pipe = MoviePipe::new(
        "/home/sobczal/Downloads/pgs.mp4".to_string(),
        "/home/sobczal/Downloads/output.mp4".to_string(),
        Box::new(MedianPipeline::new(
            MedianPipelineOptions {
                radius: 7,
            }
        )),
    );

    movie_pipe.process();

    // let processor = MedianProcessor::with_options(MedianProcessorOptions { radius: 3 });
    // let image = processor.process(image);
    // 
    // let duration = start.elapsed();
    // println!("Time elapsed in processing fast_image is: {:?}", duration);
    // 
    // image
    //     .write_to_file("/home/sobczal/Downloads/output.png")
    //     .unwrap();
}
