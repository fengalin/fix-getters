//! `fix-getters` global level `Error`.

use std::fmt::{self, Display};
use std::{io, path::PathBuf};

/// `fix-getters` global level `Error`.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    CheckEntry(rules::dir_entry::CheckError),
    CreateDir(PathBuf, io::Error),
    ReadDir(PathBuf, io::Error),
    ReadEntry(PathBuf, io::Error),
    ReadFile(PathBuf, io::Error),
    WriteFile(PathBuf, io::Error),
    ParseFile(ParseFileError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match self {
            CheckEntry(err) => err.fmt(f),
            CreateDir(path, err) => write!(f, "Unable to create dir {:?} {}", path, err),
            ReadDir(path, err) => write!(f, "Unable to read dir {:?}: {}", path, err),
            ReadEntry(path, err) => write!(f, "Unable to read dir entry {:?}: {}", path, err),
            ReadFile(path, err) => write!(f, "Unable to read file {:?}: {}", path, err),
            WriteFile(path, err) => write!(f, "Unable to write file {:?}: {}", path, err),
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
    filepath: PathBuf,
    source_code: String,
}

impl ParseFileError {
    pub fn new(error: syn::Error, filepath: PathBuf, source_code: String) -> Self {
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
