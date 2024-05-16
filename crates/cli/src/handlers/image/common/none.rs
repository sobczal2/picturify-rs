use clap::ArgMatches;
use crate::error::CliPicturifyResult;
use crate::handlers::common::handler::CommandHandler;
use crate::handlers::common::image_io::{read_image, write_image};

pub struct NoneCommandHandler;

impl CommandHandler for NoneCommandHandler {
    fn handle(args: ArgMatches) -> CliPicturifyResult<()> {
        let image = read_image(args.clone())?;

        write_image(image, args.clone())?;

        Ok(())
    }
}