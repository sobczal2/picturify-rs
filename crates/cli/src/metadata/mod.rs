const CLI_NAME: &str = "picturify-cli";
const CLI_VERSION: &str = "0.1.0";
const CORE_NAME: &str = "picturify-core";
const CORE_VERSION: &str = "0.1.0";
const PROCESSING_NAME: &str = "picturify-processing-bench";
const PROCESSING_VERSION: &str = "0.1.0";

pub struct PicturifyMetadata {
    pub cli_name: String,
    pub cli_version: String,
    pub core_name: String,
    pub core_version: String,
    pub processing_name: String,
    pub processing_version: String,
}

pub fn get_metadata() -> PicturifyMetadata {
    PicturifyMetadata {
        cli_name: CLI_NAME.to_string(),
        cli_version: CLI_VERSION.to_string(),
        core_name: CORE_NAME.to_string(),
        core_version: CORE_VERSION.to_string(),
        processing_name: PROCESSING_NAME.to_string(),
        processing_version: PROCESSING_VERSION.to_string(),
    }
}
