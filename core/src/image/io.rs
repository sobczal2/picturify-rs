use crate::error::{PicturifyError, PicturifyResult};

pub trait ReadFromFile {
    fn read_from_file(path: &str) -> PicturifyResult<Box<Self>>;
}

pub trait WriteToFile {
    fn write_to_file(&self, path: &str) -> PicturifyResult<()>;
}
