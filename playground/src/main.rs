use std::time::Instant;

use picturify_core::image::fast_image::FastImage;
use picturify_core::image::io::{ReadFromFile, WriteToFile};
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::color::negative::{NegativeProcessor, NegativeProcessorOptions};

fn main() {
    let fast_image = FastImage::read_from_file("/home/sobczal/Downloads/silnik2.jpg").unwrap();
    let image = *fast_image;
    let start = Instant::now();
    
    let processor = NegativeProcessor::with_options(NegativeProcessorOptions {
        use_fast_approximation: false,
    });
    let image = processor.process(image);
    
    let duration = start.elapsed();
    println!("Time elapsed in processing image is: {:?}", duration);

    image
        .write_to_file("/home/sobczal/Downloads/slow.png")
        .unwrap();
}