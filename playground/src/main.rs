use picturify_core::image::fast_image::FastImage;
use picturify_core::image::io::{ReadFromFile, WriteToFile};
use picturify_processing::common::execution::{ExecutionPlan};
use picturify_processing::common::process::Processor;
use std::time::Instant;
use picturify_processing::common::channel::{ChannelSelector, RgbaChannelSelector};
use picturify_processing::processors::color::negative::NegativeProcessor;
use picturify_processing::processors::color::sepia::SepiaProcessor;

fn main() {
    let mut image = FastImage::read_from_file("/home/sobczal/Downloads/blade-runner.jpg").unwrap();
    let mut neagative_processor = NegativeProcessor::new();
    neagative_processor
        .set_execution_plan(ExecutionPlan::Cpu(Default::default()))
        .unwrap();
    
    neagative_processor.set_channel_selector(ChannelSelector::Rgba(RgbaChannelSelector::new(
        false, true, false, false,
    ))).unwrap();
    
    let sepia_processor = SepiaProcessor::new();
    
    let start = Instant::now();
    let processed_image = neagative_processor.process(*image);
    let duration = start.elapsed();
    
    println!("Time elapsed in processing is: {:?}", duration);
    
    processed_image
        .write_to_file("/home/sobczal/Downloads/processed.png")
        .unwrap();
}
