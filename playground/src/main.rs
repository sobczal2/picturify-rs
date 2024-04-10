use std::time::Instant;

use picturify_core::fast_image::fast_image::FastImage;
use picturify_core::fast_image::io::{ReadFromFile, WriteToFile};
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::noise::median::{MedianProcessor, MedianProcessorOptions};

fn main() {
    let fast_image = FastImage::read_from_file("/home/sobczal/Downloads/ryan_mid_res.jpg").unwrap();
    let image = *fast_image;
    let start = Instant::now();

    let processor = MedianProcessor::with_options(MedianProcessorOptions { radius: 3 });
    let image = processor.process(image);

    let duration = start.elapsed();
    println!("Time elapsed in processing fast_image is: {:?}", duration);

    image
        .write_to_file("/home/sobczal/Downloads/output.png")
        .unwrap();
}
