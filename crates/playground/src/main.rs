use std::time::Instant;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};
use picturify_core::core::fast_image::FastImage;
use picturify_core::core::io::{ReadFromFile, WriteToFile};
use picturify_core::threading::progress::Progress;
use picturify_processing::common::processors::CpuProcessor;
use picturify_processing::processors::edge::canny::{CannyEdgeDetectionType, CannyProcessor, CannyProcessorOptions};
use picturify_processing::processors::geometry::scale::{ScaleProcessor, ScaleProcessorOptions, ScaleStrategy};

fn main() {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
        .expect("Failed to initialize logger");
    
    run_image();
    // run_movie();
}

#[allow(dead_code)]
fn run_movie() {
    // let custom_pipeline = CustomPipeline::new(|core: FastImage| {
    //     let core = core;
    //     let processor = MedianProcessor::with_options(MedianProcessorOptions { radius: 3 });
    //     let core = processor.process(core);
    //
    //     let processor = SobelRgbProcessor::new();
    //     let core = processor.process(core);
    //
    //     let processor = RemappingProcessor::with_options(RemappingProcessorOptions {
    //         function: RemappingFunction::Logarithmic { factor: 1.025 },
    //     });
    //     let core = processor.process(core);
    //
    //     let processor = ThresholdProcessor::with_options(ThresholdProcessorOptions {
    //         red_threshold: 128,
    //         green_threshold: 128,
    //         blue_threshold: 128,
    //     });
    //
    //     processor.process(core)
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

#[allow(dead_code)]
fn run_image() {
    let image = FastImage::read_from_file("/home/sobczal/Downloads/alien2.png").unwrap();
    
    // let processor = ScaleProcessor::new(ScaleProcessorOptions {
    //     size: (750, 1000).into(),
    //     strategy: ScaleStrategy::NearestNeighbor
    // });
    // 
    // let image = processor.process(image, Progress::new()).unwrap();
    // 
    let processor = CannyProcessor::new(CannyProcessorOptions {
        sigma: 1.0f32,
        radius: 2,
        edge_detection_type: CannyEdgeDetectionType::Sobel,
        low_threshold: 0.03f32,
        high_threshold: 0.07f32,
    });
        
    
    let start = Instant::now();
    let image = processor.process(image, Progress::new()).unwrap();
    let duration = start.elapsed();
    println!("Time elapsed in grayscale is: {:?}", duration);
    image
        .write_to_file("/home/sobczal/Downloads/output.png")
        .unwrap();
}
