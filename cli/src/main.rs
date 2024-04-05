extern crate picturify_core;
extern crate picturify_processing;
extern crate image;

use image::io::Reader;
use picturify_processing::core::processor::Processor;
use picturify_processing::noise_reduction::kuwahara_filter::{KuwaharaFilter, KuwaharaFilterOptions};

fn main() -> Result<(), String> {
    let image = Reader::open("/home/sobczal/Downloads/bambi.jpg").unwrap()
        .decode().unwrap();


    
    let processor = KuwaharaFilter::new(KuwaharaFilterOptions{
        window_size: 5,
    });
    let start_time = std::time::Instant::now();
    let new_image = processor.process(image);
    let duration = start_time.elapsed();

    println!("Time taken to process: {:?}", duration);
    
    new_image.save("/home/sobczal/Downloads/processed.png").map_err(|e| e.to_string())?;
    
    Ok(())
}
