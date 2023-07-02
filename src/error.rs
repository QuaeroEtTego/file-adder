use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

use self::Error::*;

#[derive(Debug)]
pub enum Error {
    Interaction,
    InvalidPath,
    DirectoryCreate,
    FileOpen,
    FileWrite,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Interaction => f.write_str("Interaction failed, cannot read user input"),
            InvalidPath => f.write_str("Path of the output file is invalid"),
            DirectoryCreate => {
                f.write_str("Could not create all necessary directories from the path")
            }
            FileOpen => f.write_str("Failed to create/open the output file"),
            FileWrite => f.write_str("Failed to write all data in the output file"),
        }
    }
}

impl StdError for Error {}
