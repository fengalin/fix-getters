//! `fix-getters` global level `Error`.

use std::fmt::{self, Display};
use std::io;

/// `fix-getters` global level `Error`.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    CheckEntry(rules::dir_entry::CheckError),
    CreateDir(String, io::Error),
    ReadDir(io::Error),
    ReadEntry(io::Error),
    ReadFile(io::Error),
    WriteFile(io::Error),
    ParseFile(ParseFileError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match self {
            CheckEntry(err) => write!(f, "unable to check dir entry: {}", err),
            CreateDir(name, err) => write!(f, "unable to create dir {}: {}", name, err),
            ReadDir(err) => write!(f, "unable to read dir: {}", err),
            ReadEntry(err) => write!(f, "unable to read dir entry: {}", err),
            ReadFile(err) => write!(f, "unable to read file: {}", err),
            WriteFile(err) => write!(f, "unable to write file: {}", err),
            ParseFile(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<rules::dir_entry::CheckError> for Error {
    fn from(err: rules::dir_entry::CheckError) -> Self {
        Error::CheckEntry(err)
    }
}

/// Rust code parser error wrapper.
#[derive(Debug)]
pub struct ParseFileError {
    error: syn::Error,
    filepath: std::path::PathBuf,
    source_code: String,
}

impl ParseFileError {
    pub fn new(error: syn::Error, filepath: std::path::PathBuf, source_code: String) -> Self {
        ParseFileError {
            error,
            filepath,
            source_code,
        }
    }
}

impl Display for ParseFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "failed to parse file {:?}: {:?}\n\t{}",
            self.filepath, self.error, self.source_code
        )
    }
}

impl std::error::Error for ParseFileError {}

impl From<ParseFileError> for super::Error {
    fn from(err: ParseFileError) -> Self {
        Error::ParseFile(err)
    }
}
