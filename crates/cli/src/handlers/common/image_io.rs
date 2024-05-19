use crate::commands::common::arg::ArgType;
use crate::error::{CliPicturifyError, CliPicturifyResult};
use clap::ArgMatches;
use log::debug;
use picturify_core::fast_image::io::{ReadFromFile, WriteToFile};
use picturify_core::fast_image::FastImage;
use std::time::Instant;

pub fn read_image(args: ArgMatches) -> CliPicturifyResult<FastImage> {
    let input = args
        .get_one::<String>(ArgType::Input.to_id())
        .ok_or(CliPicturifyError::MissingArgument("Input".to_string()))?;

    let read_start = Instant::now();
    let image = FastImage::read_from_file(input)?;
    let image = *image;
    let read_elapsed_ms = read_start.elapsed().as_millis();
    debug!("Reading fast_image took {}ms", read_elapsed_ms);

    Ok(image)
}

pub fn write_image(image: FastImage, args: ArgMatches) -> CliPicturifyResult<()> {
    let output = args
        .get_one::<String>(ArgType::Output.to_id())
        .ok_or(CliPicturifyError::MissingArgument("Output".to_string()))?;

    let write_start = Instant::now();
    image.write_to_file(output)?;
    let write_elapsed_ms = write_start.elapsed().as_millis();
    debug!("Writing fast_image took {}ms", write_elapsed_ms);

    Ok(())
}
