use std::fs::read_dir;
use picturify_core::image::fast_image::FastImage;
use picturify_core::image::io::{ReadFromFile, WriteToFile};
use picturify_processing::common::execution::{ExecutionPlan};
use picturify_processing::common::process::Processor;
use std::time::Instant;
use picturify_core::image::pixel::RgbaPixel;
use picturify_processing::common::channel::{ChannelSelector, HslaChannelSelector, RgbaChannelSelector};
use picturify_processing::processors::color::negative::NegativeProcessor;
use picturify_processing::processors::color::sepia::SepiaProcessor;
use picturify_processing::processors::edge::sobel::SobelOperatorProcessor;

fn main() {
    let read_dir = read_dir("/home/sobczal/Documents/picturify-examples/frames").unwrap();

    for entry in read_dir {
        let entry = entry.unwrap();
        let path = entry.path();
        let mut sobel_processor = SobelOperatorProcessor::new();

        let mut fast_image = FastImage::read_from_file(&path.to_str().unwrap().to_string()).unwrap();
        let image = *fast_image;
        let start = Instant::now();
        // sobel_processor.set_channel_selector(ChannelSelector::Rgba(RgbaChannelSelector::new(
        //     true,
        //     false,
        //     false,
        //     false,
        // ))).unwrap();
        // sobel_processor.set_magnitude_mapping(|old_pixel, magnitude_squared| {
        //     let magnitude = if magnitude_squared > 0.01 {
        //         255
        //     } else {
        //         0
        //     };
        //     RgbaPixel {
        //         red: magnitude,
        //         green: old_pixel.green,
        //         blue: old_pixel.blue,
        //         alpha: old_pixel.alpha,
        //     }
        // }).unwrap();
        // let image = sobel_processor.process(image);
        // 
        // sobel_processor.set_channel_selector(ChannelSelector::Rgba(RgbaChannelSelector::new(
        //     false,
        //     true,
        //     false,
        //     false,
        // ))).unwrap();
        // sobel_processor.set_magnitude_mapping(|old_pixel, magnitude_squared| {
        //     let magnitude = if magnitude_squared > 0.01 {
        //         255
        //     } else {
        //         0
        //     };
        //     RgbaPixel {
        //         red: old_pixel.red,
        //         green: magnitude,
        //         blue: old_pixel.blue,
        //         alpha: old_pixel.alpha,
        //     }
        // }).unwrap();
        // let image = sobel_processor.process(image);
        // 
        // sobel_processor.set_channel_selector(ChannelSelector::Rgba(RgbaChannelSelector::new(
        //     false,
        //     false,
        //     true,
        //     false,
        // ))).unwrap();
        // 
        // sobel_processor.set_magnitude_mapping(|old_pixel, magnitude_squared| {
        //     let magnitude = if magnitude_squared > 0.01 {
        //         255
        //     } else {
        //         0
        //     };
        //     RgbaPixel {
        //         red: old_pixel.red,
        //         green: old_pixel.green,
        //         blue: magnitude,
        //         alpha: old_pixel.alpha,
        //     }
        // }).unwrap();
        // 
        // let image = sobel_processor.process(image);

        let sepia_processor = SepiaProcessor::new();
        
        let image = sepia_processor.process(image);
        
        let duration = start.elapsed();
        println!("Time elapsed in processing image: {:?}. Image name: {}", duration, path.to_str().unwrap());

        image.write_to_file(&format!("/home/sobczal/Documents/picturify-examples/frames-output/{}", path.file_name().unwrap().to_str().unwrap())).unwrap();
    }
}
