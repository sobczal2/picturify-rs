use std::path::PathBuf;
use std::time::Instant;

use clap::ArgMatches;

use picturify_core::core::fast_image::FastImage;
use picturify_core::core::io::{ReadFromFile, WriteToFile};
use picturify_core::log_debug;

use crate::commands::common::args::common::{InputArg, OutputArg, PicturifyArg};
use crate::error::{CliPicturifyError, CliPicturifyResult};

pub fn read_image(args: ArgMatches) -> CliPicturifyResult<FastImage> {
    let input = args
        .get_one::<PathBuf>(InputArg::id())
        .expect("Input argument is required");

    let read_start = Instant::now();
    let image = FastImage::read_from_file(input).map_err(|_| CliPicturifyError::Command("error reading image".to_string()))?;
    let read_elapsed_ms = read_start.elapsed().as_millis();
    log_debug!(format!("Reading image took {}ms", read_elapsed_ms));

    Ok(image)
}

pub fn write_image(image: FastImage, args: ArgMatches) -> CliPicturifyResult<()> {
    let output = args
        .get_one::<PathBuf>(OutputArg::id())
        .expect("Input argument is required");

    let write_start = Instant::now();
    image.write_to_file(output).map_err(|_| CliPicturifyError::Command("error writing image".to_string()))?;
    let write_elapsed_ms = write_start.elapsed().as_millis();
    log_debug!(format!("Writing image took {}ms", write_elapsed_ms));

    Ok(())
}
