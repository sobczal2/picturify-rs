use std::path::Path;
use image::{ColorType, DynamicImage, GenericImage, GenericImageView, RgbaImage};
use ndarray::Array3;
use video_rs::{Decoder, Encoder, Frame, Time};
use video_rs::encode::Settings;
use picturify_processing::core::processor::{Processor, ProcessorRunner};

pub struct MoviePipeline {
    input_path: String,
    output_path: String,
    processors: Vec<Box<dyn Processor>>,
    processor_runner: ProcessorRunner,
}

impl MoviePipeline {
    pub fn new(input_path: String, output_path: String, processors: Vec<Box<dyn Processor>>, processor_runner: ProcessorRunner) -> Self {
        MoviePipeline {
            input_path,
            output_path,
            processors,
            processor_runner,
        }
    }

    pub fn process(&self) {
        let source = Path::new(&self.input_path);
        let destination = Path::new(&self.output_path);

        let mut decoder = Decoder::new(source).unwrap();
        let mut encoder: Option<Encoder> = None;

        let frame_rate = decoder.frame_rate();

        let mut frame_count = 0;

        let mut current_time = Time::zero();

        let mut time_per_frame = Time::from_nth_of_a_second(frame_rate as usize);

        for frame in decoder.decode_iter() {
            if let Ok((_, frame)) = frame {
                let mut image = frame_to_image(frame);
                for processor in &self.processors {
                    image = processor.process(image, &self.processor_runner);
                }

                if encoder.is_none() {
                    let settings = Settings::default().with_frame_rate(frame_rate);
                    encoder = Some(Encoder::new(destination, settings).unwrap());
                }

                encoder.unwrap().encode(&image_to_frame(image), &current_time).unwrap();

                current_time = current_time.aligned_with(&time_per_frame).add();
            }
        }
    }
}

fn frame_to_image(value: Frame) -> DynamicImage {
    let frame_width = value.dim().0 as u32;
    let frame_height = value.dim().1 as u32;
    let mut image = DynamicImage::new(frame_width, frame_height, ColorType::Rgba8);
    value.rows().into_iter().for_each(|row| {
        row.into_iter().for_each(|pixel| {
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let a = pixel[3];

            image.put_pixel(r as u32, g as u32, image::Rgba([r, g, b, a]));
        });
    });
    image
}

fn image_to_frame(value: DynamicImage) -> Frame {
    let frame_width = value.width() as usize;
    let frame_height = value.height() as usize;
    let mut frame = Array3::zeros((frame_width, frame_height, 4));
    for y in 0..frame_height {
        for x in 0..frame_width {
            let pixel = value.get_pixel(x as u32, y as u32);
            frame[[x, y, 0]] = pixel[0];
            frame[[x, y, 1]] = pixel[1];
            frame[[x, y, 2]] = pixel[2];
            frame[[x, y, 3]] = pixel[3];
        }
    }
    frame
}
