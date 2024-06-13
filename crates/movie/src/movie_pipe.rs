use picturify_core::core::fast_image::FastImage;
use std::fs::remove_file;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, RwLock};

use crate::error::{MoviePicturifyError, MoviePicturifyResult};
use crate::progress::{MovieProgress, ProgressStage};
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use picturify_pipeline::pipeline::Pipeline;

pub struct MoviePipe;

impl MoviePipe {
    pub fn process(
        source: String,
        destination: String,
        pipeline: Box<dyn Pipeline>,
        progress: Arc<RwLock<MovieProgress>>,
    ) -> MoviePicturifyResult<()> {
        progress.read().unwrap().set_stage(ProgressStage::Probe);
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
                &source,
            ])
            .output()
            .map_err(|_| MoviePicturifyError::FfprobeNotFound)?;

        let ffprobe_output_str = String::from_utf8_lossy(&ffprobe_output.stdout);
        let mut iter = ffprobe_output_str.trim().split(',');
        if iter.clone().count() != 4 {
            return Err(MoviePicturifyError::FfprobeFailed);
        }
        let width: u32 = iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_| MoviePicturifyError::FfprobeFailed)?;
        let height: u32 = iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_| MoviePicturifyError::FfprobeFailed)?;
        let mut framerate_iter = iter.next().unwrap().split('/');
        let framerate_numerator: f32 = framerate_iter
            .next()
            .unwrap()
            .parse::<f32>()
            .map_err(|_| MoviePicturifyError::FfprobeFailed)?;
        let framerate_denominator: f32 = framerate_iter
            .next()
            .unwrap()
            .parse::<f32>()
            .map_err(|_| MoviePicturifyError::FfprobeFailed)?;
        let framerate = framerate_numerator / framerate_denominator;
        let frame_count = iter
            .next()
            .unwrap()
            .parse()
            .map_err(|_| MoviePicturifyError::FfprobeFailed)?;

        progress.read().unwrap().setup(frame_count);

        let intermediete_file = format!("{}_intermediate.mp4", &source);

        let ffmpeg_read_process = Command::new("ffmpeg")
            .args([
                "-i",
                &source,
                "-vf",
                "format=rgba",
                "-f",
                "rawvideo",
                "pipe:1",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| MoviePicturifyError::FfmpegNotFound)?;

        let mut ffmpeg_stdout = ffmpeg_read_process
            .stdout
            .ok_or(MoviePicturifyError::FfmpegFailed)?;

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
            .map_err(|_| MoviePicturifyError::FfmpegNotFound)?;

        let mut ffmpeg_stdin = ffmpeg_process
            .stdin
            .ok_or(MoviePicturifyError::FfmpegFailed)?;

        let mut buffer = vec![0u8; width as usize * height as usize * 4];

        progress.read().unwrap().set_stage(ProgressStage::Process);
        while ffmpeg_stdout.read_exact(&mut buffer).is_ok() {
            progress.read().unwrap().increment();
            let image = FastImage::from_rgba_vec((width, height).into(), buffer.clone());
            let progress = Some(PipelineProgress::new());
            let processed_image = pipeline.run(image, progress);
            ffmpeg_stdin
                .write_all(&processed_image.to_rgba_vec())
                .map_err(|_| MoviePicturifyError::FfmpegFailed)?;
        }

        ffmpeg_process.stdin = Some(ffmpeg_stdin);
        ffmpeg_process
            .wait()
            .map_err(|_| MoviePicturifyError::FfmpegFailed)?;

        progress.read().unwrap().set_stage(ProgressStage::Merge);
        let mut add_audio = Command::new("ffmpeg")
            .args([
                "-y",
                "-i",
                &source,
                "-i",
                intermediete_file.as_str(),
                "-c",
                "copy",
                "-map",
                "0:a",
                "-map",
                "1:v",
                &destination,
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| MoviePicturifyError::FfmpegNotFound)?;

        add_audio
            .wait()
            .map_err(|_| MoviePicturifyError::FfmpegFailed)?;
        remove_file(intermediete_file).map_err(|_| MoviePicturifyError::FfmpegFailed)?;

        progress.read().unwrap().set_stage(ProgressStage::Finish);

        Ok(())
    }
}
