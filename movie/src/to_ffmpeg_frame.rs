use ffmpeg_next::format::Pixel;
use ffmpeg_next::frame::Video;
use picturify_core::fast_image::fast_image::FastImage;
use picturify_core::fast_image::util::cord_2d_to_1d;

pub trait ToFFmpegFrame {
    fn to_frame(&self) -> Video;
}

impl ToFFmpegFrame for FastImage {
    fn to_frame(&self) -> Video {
        let width = self.get_width();
        let height = self.get_height();
        let mut frame = Video::new(
            Pixel::YUV420P,
            width as u32,
            height as u32,
        );
        let frame_data = frame.data_mut(0);
        
        for y in 0..height {
            for x in 0..width {
                let pixel = self.get_image_pixel(x, y);
                let yuv_pixel = pixel[0];
                frame_data[cord_2d_to_1d(x, y, width)] = yuv_pixel;
            }
        }
        frame
    }
}