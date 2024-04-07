use picturify_core::image::fast_image::FastImage;
use picturify_core::image::io::{ReadFromFile, WriteToFile};
use picturify_processing::common::execution::{ExecutionPlan, MultiThreadCpuOptions};
use picturify_processing::common::process::Processor;
use picturify_processing::processors::color::sepia::SepiaProcessor;
use std::time::Instant;

fn main() {
    let mut image = FastImage::read_from_file("/home/sobczal/Downloads/ava2.jpg").unwrap();
    let mut processor = SepiaProcessor::new();
    processor
        .set_execution_plan(ExecutionPlan::MultiThreadCpu(Default::default()))
        .unwrap();

    let start = Instant::now();
    let processed_image = processor.process(*image);
    let duration = start.elapsed();

    println!("Time elapsed in processing is: {:?}", duration);

    processed_image
        .write_to_file("/home/sobczal/Downloads/ava_sepia_rs.png")
        .unwrap();
}
