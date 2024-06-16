use std::path::Path;
use crate::error::PicturifyResult;

pub trait ReadFromFile
    where
        Self: Sized,
{
    fn read_from_file<'a, P>(path: P) -> PicturifyResult<Self>
        where
            P: AsRef<Path>;
}

pub trait WriteToFile {
    fn write_to_file<P>(&self, path: P) -> PicturifyResult<()>
        where
            P: AsRef<Path>;
}
