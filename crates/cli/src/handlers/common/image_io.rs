use std::path::PathBuf;
use std::time::Instant;

use clap::ArgMatches;
use log::debug;

use picturify_core::core::fast_image::FastImage;
use picturify_core::core::io::{ReadFromFile, WriteToFile};

use crate::commands::common::args::common::{InputArg, OutputArg, PicturifyArg};
use crate::error::{CliPicturifyResult};

pub fn read_image(args: ArgMatches) -> CliPicturifyResult<FastImage> {
    let input = args
        .get_one::<PathBuf>(InputArg::id())
        .expect("Input argument is required");

    let read_start = Instant::now();
    let image = FastImage::read_from_file(input)?;
    let read_elapsed_ms = read_start.elapsed().as_millis();
    debug!("Reading core took {}ms", read_elapsed_ms);

    Ok(image)
}

pub fn write_image(image: FastImage, args: ArgMatches) -> CliPicturifyResult<()> {
    let output = args
        .get_one::<PathBuf>(OutputArg::id())
        .expect("Input argument is required");

    let write_start = Instant::now();
    image.write_to_file(output)?;
    let write_elapsed_ms = write_start.elapsed().as_millis();
    debug!("Writing core took {}ms", write_elapsed_ms);

    Ok(())
}
