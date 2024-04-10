use std::time::Instant;

use picturify_core::image::fast_image::FastImage;
use picturify_core::image::io::{ReadFromFile, WriteToFile};
use picturify_processing::common::execution::Processor;
use picturify_processing::processors::noise::kuwahara::{
    KuwaharaProcessor, KuwaharaProcessorOptions,
};

fn main() {
    let fast_image = FastImage::read_from_file("/home/sobczal/Downloads/silnik2.jpg").unwrap();
    let image = *fast_image;
    let start = Instant::now();

    let processor = KuwaharaProcessor::with_options(KuwaharaProcessorOptions { quadrant_size: 3 });
    let image = processor.process(image);

    let duration = start.elapsed();
    println!("Time elapsed in processing image is: {:?}", duration);

    image
        .write_to_file("/home/sobczal/Downloads/output.png")
        .unwrap();
}
