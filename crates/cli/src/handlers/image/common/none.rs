use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image_io::{read_image, write_image};
use clap::ArgMatches;

pub struct NoneCommandHandler;

impl CommandHandler for NoneCommandHandler {
    fn handle(&self, args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;

        write_image(image, args.clone())?;

        Ok(())
    }
}
