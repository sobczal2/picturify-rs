use picturify_core::core::fast_image::FastImage;
use picturify_core::core::io::WriteToFile;
use picturify_core::geometry::size::Size;

fn main() {
    let image = FastImage::empty(Size::new(100, 100));
    image
        .write_to_file(
            "/home/sobczal/Devel/rust/picturify-rs/tests/cli-e2e/assets/sample_100x100.png",
        )
        .unwrap()
    // run_image();
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
    // let core = *FastImage::read_from_file("/home/sobczal/Downloads/large.jpg").unwrap();
    //
    // let processor = CropProcessor::new().with_options(CropProcessorOptions {
    //     x: 100,
    //     y: 100,
    //     width: 2000,
    //     height: 2000,
    // });
    //
    // let start = Instant::now();
    // let core = processor.process(core, Arc::new(RwLock::new(Progress::new())));
    // let duration = start.elapsed();
    // println!("Time elapsed in grayscale is: {:?}", duration);
    // core
    //     .write_to_file("/home/sobczal/Downloads/output.png")
    //     .unwrap();
}
