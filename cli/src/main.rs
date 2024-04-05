extern crate picturify_core;
extern crate picturify_processing;
extern crate image;

use image::io::Reader;
use picturify_processing::color::sepia::{SepiaFilter, SepiaFilterOptions};
use picturify_processing::core::processor::{Processor, ProcessorRunner};

fn main() -> Result<(), String> {
    let start_time = std::time::Instant::now();
    let mut image = Reader::open("/home/sobczal/Downloads/test2.png").unwrap()
        .decode().unwrap();
    
    // image = DynamicImage::ImageLuma8(image.to_luma8());
    
    // let processor = KuwaharaFilter::new(KuwaharaFilterOptions{
    //     window_size: 5,
    // });

    let processor = SepiaFilter::new(SepiaFilterOptions{});
    
    
    
    image = processor.process(image, &ProcessorRunner::RayonCpu(Default::default()));


    image.save("/home/sobczal/Downloads/processed.png").map_err(|e| e.to_string())?;
    let duration = start_time.elapsed();
    println!("Time taken to read, process and write: {:?}", duration);
    
    Ok(())
}
