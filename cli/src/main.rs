extern crate picturify_core;
extern crate picturify_processing;

use picturify_core::io::image::{read_image_from_file, write_image_to_file};
use picturify_core::pixel::color::ColorSpace;
use picturify_processing::core::processor::Processor;
use picturify_processing::noise_reduction::kuwahara_filter::{KuwaharaFilter, KuwaharaFilterOptions};

fn main() -> Result<(), String> {
    let mut image = read_image_from_file(std::path::Path::new("/home/sobczal/Downloads/ryan.jpg")).unwrap();


    
    let processor = KuwaharaFilter::new(KuwaharaFilterOptions{
        window_size: 10,
    });
    let start_time = std::time::Instant::now();
    let new_image = processor.process(image);
    let duration = start_time.elapsed();

    println!("Time taken to process: {:?}", duration);
    write_image_to_file(&new_image, std::path::Path::new("/home/sobczal/Downloads/test2_copy.png")).unwrap();
    
    Ok(())
}
