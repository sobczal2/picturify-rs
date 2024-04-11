use std::io::{Read, Write};
use std::process::{ChildStdout, Command, Stdio};
use std::rc::Rc;
use ffmpeg_next::{codec, decoder, Dictionary, picture};
use ffmpeg_next::ffi::avcodec_alloc_context3;
use ffmpeg_next::format::Pixel;
use ffmpeg_next::frame::Video;
use ffmpeg_next::packet::Mut;
use ffmpeg_next::software::scaling::{Context, Flags};
use picturify_core::fast_image::fast_image::FastImage;
use picturify_core::fast_image::io::WriteToFile;
use picturify_core::image;
use picturify_pipeline::pipeline::Pipeline;
use crate::from_ffmpeg_frame::FromFFmpegFrame;
use crate::to_ffmpeg_frame::ToFFmpegFrame;
use crate::transcode_x264::transcode_x264;

pub struct MoviePipe {
    pub source: String,
    pub destination: String,
    pub pipeline: Box<dyn Pipeline>,
}

impl MoviePipe {
    pub fn new(
        source: String,
        destination: String,
        pipeline: Box<dyn Pipeline>,
    ) -> Self {
        Self {
            source,
            destination,
            pipeline,
        }
    }

    pub fn process(&self) {
        let now = std::time::Instant::now();
        let ffprobe_output = Command::new("ffprobe")
            .args(&["-v", "error", "-select_streams", "v:0", "-show_entries", "stream=width,height,r_frame_rate", "-of", "csv=p=0", &self.source])
            .output().unwrap();

        // Parse ffprobe output to get video width, height, and framerate
        let ffprobe_output_str = String::from_utf8_lossy(&ffprobe_output.stdout);
        let mut iter = ffprobe_output_str.trim().split(',');
        let width: u32 = iter.next().unwrap().parse().unwrap();
        let height: u32 = iter.next().unwrap().parse().unwrap();
        let mut framerate_iter = iter.next().unwrap().split('/');
        let framerate: f32 = framerate_iter.next().unwrap().parse::<f32>().unwrap() / framerate_iter.next().unwrap().parse::<f32>().unwrap();
        
        let intermediete_file = format!("{}_intermediate.mp4", &self.source);

        // Spawn ffmpeg process to read MP4 file and output raw RGBA pixel stream to stdout
        let mut ffmpeg_read_process = Command::new("ffmpeg")
            .args(&["-i", &self.source, "-vf", "format=rgba", "-f", "rawvideo", "pipe:1"])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn().unwrap();

        // Get stdout handle for ffmpeg process
        let mut ffmpeg_stdout = ffmpeg_read_process.stdout.expect("Failed to open stdout for ffmpeg process");

        // Spawn another ffmpeg process to read raw RGBA pixel stream from stdin and write modified frames to output.mp4
        let mut ffmpeg_process = Command::new("ffmpeg")
            .args(&["-y", "-f", "rawvideo",
                "-pixel_format", "rgba", "-video_size",
                &format!("{}x{}", width, height), "-framerate", &format!("{}", framerate),
                "-i", "pipe:0", "-c:v", "libx264", "-pix_fmt",
                "yuv420p",
                intermediete_file.as_str()])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn().unwrap();

        // Take ownership of ffmpeg_process.stdin
        let mut ffmpeg_stdin = ffmpeg_process.stdin.take().expect("Failed to open stdin for ffmpeg process");

        // Create a buffer to hold data read from ffmpeg stdout
        let mut buffer = vec![0u8; width as usize * height as usize * 4];

        // Read data from the first ffmpeg process stdout and write it to the second ffmpeg process stdin
        let mut frame_count = 0;
        loop {
            match ffmpeg_stdout.read_exact(&mut buffer) {
                Ok(_) => {
                    frame_count += 1;
                    println!("Processing frame {}", frame_count);
                    let image = FastImage::from_rgba_vec(width as usize, height as usize, buffer.clone());
                    let processed_image = self.pipeline.run(image);
                    ffmpeg_stdin.write_all(&processed_image.to_rgba_vec()).unwrap();
                }
                Err(_) => {
                    // If an error occurs (e.g., end of stream), break the loop
                    break;
                }
            }
        }

        // Reassign ffmpeg_process.stdin with the modified stdin
        ffmpeg_process.stdin = Some(ffmpeg_stdin);

        // Wait for the first ffmpeg process to finish
        ffmpeg_process.wait().unwrap();
        
        let elapsed = now.elapsed();
        println!("Processed {} frames in {} seconds", frame_count, elapsed.as_secs_f32());
        println!("Adding audio to video");
        let mut add_audio = Command::new("ffmpeg")
            .args(&["-y", "-i", &self.source, "-i", intermediete_file.as_str(), "-c", "copy", "-map", "0:a", "-map", "1:v", &self.destination])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn().unwrap();
        
        
        add_audio.wait().unwrap();
        println!("Finished processing video");
        
        std::fs::remove_file(intermediete_file).unwrap();
        println!("Finished processing video");
    }
}