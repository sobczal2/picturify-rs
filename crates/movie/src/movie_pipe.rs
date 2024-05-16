use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, RwLock};
use std::time::Instant;

use picturify_core::fast_image::FastImage;
use picturify_core::threading::progress::Progress;
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use picturify_pipeline::pipeline::Pipeline;

pub struct MoviePipe {
    pub source: String,
    pub destination: String,
    pub pipeline: Box<dyn Pipeline>,
}

impl MoviePipe {
    pub fn new(source: String, destination: String, pipeline: Box<dyn Pipeline>) -> Self {
        Self {
            source,
            destination,
            pipeline,
        }
    }

    pub fn process(&self) {
        let now = std::time::Instant::now();
        let ffprobe_output = Command::new("ffprobe")
            .args([
                "-v",
                "error",
                "-select_streams",
                "v:0",
                "-show_entries",
                "stream=width,height,r_frame_rate,nb_frames",
                "-of",
                "csv=p=0",
                &self.source,
            ])
            .output()
            .unwrap();

        let ffprobe_output_str = String::from_utf8_lossy(&ffprobe_output.stdout);
        let mut iter = ffprobe_output_str.trim().split(',');
        let width: u32 = iter.next().unwrap().parse().unwrap();
        let height: u32 = iter.next().unwrap().parse().unwrap();
        let mut framerate_iter = iter.next().unwrap().split('/');
        let framerate: f32 = framerate_iter.next().unwrap().parse::<f32>().unwrap()
            / framerate_iter.next().unwrap().parse::<f32>().unwrap();
        let frame_count: u64 = iter.next().unwrap().parse().unwrap();

        let intermediete_file = format!("{}_intermediate.mp4", &self.source);

        let ffmpeg_read_process = Command::new("ffmpeg")
            .args([
                "-i",
                &self.source,
                "-vf",
                "format=rgba",
                "-f",
                "rawvideo",
                "pipe:1",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        let mut ffmpeg_stdout = ffmpeg_read_process
            .stdout
            .expect("Failed to open stdout for ffmpeg process");

        let mut ffmpeg_process = Command::new("ffmpeg")
            .args([
                "-y",
                "-f",
                "rawvideo",
                "-pixel_format",
                "rgba",
                "-video_size",
                &format!("{}x{}", width, height),
                "-framerate",
                &format!("{}", framerate),
                "-i",
                "pipe:0",
                "-c:v",
                "libx264",
                "-pix_fmt",
                "yuv420p",
                intermediete_file.as_str(),
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        let mut ffmpeg_stdin = ffmpeg_process
            .stdin
            .take()
            .expect("Failed to open stdin for ffmpeg process");

        let mut buffer = vec![0u8; width as usize * height as usize * 4];

        // let progress_bar = ProgressBar::new(frame_count);
        // progress_bar.set_style(
        //     indicatif::ProgressStyle::default_bar()
        //         .template("{msg}\n[{elapsed_precise}] {bar:100.green/white} {pos}/{len} ({eta})")
        //         .unwrap(),
        // );

        let mut frame_count = 0;
        while ffmpeg_stdout.read_exact(&mut buffer).is_ok() {
            let now = Instant::now();
            frame_count += 1;
            let image =
                FastImage::from_rgba_vec(width as usize, height as usize, buffer.clone());
            let progress = Arc::new(RwLock::new(PipelineProgress::new()));
            let processed_image = self.pipeline.run(image, progress);
            ffmpeg_stdin
                .write_all(&processed_image.to_rgba_vec())
                .unwrap();
            // progress_bar.inc(1);
            let frames_per_second = 1.0 / now.elapsed().as_secs_f32();
            let speed = frames_per_second / framerate;
            // progress_bar.set_message(format!(
            //     "Speed: {:.2}x, FPS: {:.2}",
            //     speed, frames_per_second
            // ));
        }

        // progress_bar.finish();

        ffmpeg_process.stdin = Some(ffmpeg_stdin);

        ffmpeg_process.wait().unwrap();

        let elapsed = now.elapsed();
        println!(
            "Processed {} frames in {} seconds",
            frame_count,
            elapsed.as_secs_f32()
        );
        println!("Adding audio to video");
        let mut add_audio = Command::new("ffmpeg")
            .args([
                "-y",
                "-i",
                &self.source,
                "-i",
                intermediete_file.as_str(),
                "-c",
                "copy",
                "-map",
                "0:a",
                "-map",
                "1:v",
                &self.destination,
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        add_audio.wait().unwrap();
        println!("Finished processing video");

        std::fs::remove_file(intermediete_file).unwrap();
        println!("Finished processing video");
    }
}
