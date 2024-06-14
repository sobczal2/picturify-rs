use clap::ArgMatches;

use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image_io::{read_image, write_image};

pub struct PassthroughCommandHandler;

impl CommandHandler for PassthroughCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;

        write_image(image, args.clone())?;

        Ok(())
    }
}
