use std::sync::{Arc, Mutex, RwLock};
use std::thread::spawn;
use picturify_core::fast_image::io::{ReadFromFile, WriteToFile};
use picturify_core::fast_image::FastImage;

use picturify_processing::common::execution::Processor;
use picturify_processing::processors::noise::mean::{MeanProcessor, MeanProcessorOptions};
use std::time::Instant;
use picturify_core::threading::progress::Progress;

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
    let fast_image = *FastImage::read_from_file("/home/sobczal/Downloads/ryan.jpg").unwrap();

    let processor = MeanProcessor::new();

    let progress = Arc::new(RwLock::new(Progress::new()));
    let progress_clone = progress.clone();
    let finished = Arc::new(Mutex::new(false));
    let finished_clone = finished.clone();

    let thread = spawn(move || {
        while !*finished_clone.lock().unwrap() {
            {
                let read_progress = progress_clone.read().expect("Failed to lock progress");
                println!("Progress: {}/{}", read_progress.get(), read_progress.get_max());
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    let start = Instant::now();
    let fast_image = processor.process(fast_image, progress);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    {
        let mut finished_locked = finished.lock().unwrap();
        *finished_locked = true;
    }
    thread.join().unwrap();
    fast_image
        .write_to_file("/home/sobczal/Downloads/ryan_mean.png")
        .unwrap();
}