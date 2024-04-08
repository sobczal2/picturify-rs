use std::time::Instant;

use picturify_core::image::fast_image::FastImage;
use picturify_core::image::io::{ReadFromFile, WriteToFile};
use picturify_processing::common::channel::{ChannelSelector, HslaChannelSelector, HsvaChannelSelector};
use picturify_processing::common::process::Processor;
use picturify_processing::processors::color::negative::NegativeProcessor;
use picturify_processing::processors::geometry::edge_enlargement::EdgeEnlargementProcessor;

fn main() {
    let fast_image = FastImage::read_from_file("/home/sobczal/Downloads/silnik2.jpg").unwrap();
    let image = *fast_image;
    let start = Instant::now();

    let edge_enlargement_processor = EdgeEnlargementProcessor::new(100);

    let mut image = edge_enlargement_processor.process(image);

    let mut negative_processor = NegativeProcessor::new();

    negative_processor.set_channel_selector(ChannelSelector::Hsla(HslaChannelSelector::new(
        false, false, true, false,
    ))).unwrap();
    
    image = negative_processor.process(image);
    
    let duration = start.elapsed();
    println!("Time elapsed in processing image is: {:?}", duration);

    image
        .write_to_file("/home/sobczal/Downloads/output.png")
        .unwrap();
}
