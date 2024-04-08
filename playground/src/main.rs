use std::time::Instant;

use picturify_core::image::fast_image::FastImage;
use picturify_core::image::io::{ReadFromFile, WriteToFile};
use picturify_core::image::virtual_image::VirtualImage;
use picturify_processing::common::process::Processor;
use picturify_processing::processors::edge::sobel::SobelOperatorProcessor;
use picturify_processing::processors::geometry::edge_enlargement::EdgeEnlargementProcessor;

fn main() {
    let fast_image = FastImage::read_from_file("/home/sobczal/Downloads/ryan.jpg").unwrap();
    let image = *fast_image;
    let start = Instant::now();

    let edge_enlargement_processor = EdgeEnlargementProcessor::new(100);

    let mut image = edge_enlargement_processor.process(image);
    
    let sobel_operator = SobelOperatorProcessor::new();
    image = sobel_operator.process(image);
    
    let width = image.get_width();
    let height = image.get_height();
    
    image.crop(1, 1, width - 2, height - 2);
    
    let duration = start.elapsed();
    println!("Time elapsed in processing image is: {:?}", duration);

    image.write_to_file("/home/sobczal/Downloads/ducks.png").unwrap();
}
