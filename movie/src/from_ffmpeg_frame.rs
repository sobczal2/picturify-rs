use ffmpeg_next::frame::Video;
use picturify_core::fast_image::fast_image::FastImage;
use picturify_core::fast_image::util::cord_2d_to_1d;
use picturify_core::image;

pub trait FromFFmpegFrame {
    fn from_frame(frame: &Video) -> Self;
}

impl FromFFmpegFrame for FastImage {
    fn from_frame(frame: &Video) -> Self {
        let width = frame.width() as usize;
        let height = frame.height() as usize;
        let mut image = FastImage::empty(width, height);
        let frame_data = frame.data(0);
        
        for y in 0..height {
            for x in 0..width {
                let r = frame_data[cord_2d_to_1d(x, y, width)];
                image.set_image_pixel(x, y, image::Rgba([r, r, r, 255]));
            }
        }
        image
    }
}