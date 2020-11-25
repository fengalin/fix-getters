use std::fmt::{self, Display};
use std::io;

pub enum Error {
    CheckEntry(rules::dir_entry::CheckError),
    CreateDir(io::Error),
    ReadDir(io::Error),
    ReadEntry(io::Error),
    ReadFile(io::Error),
    WriteFile(io::Error),
    ParseFile {
        error: syn::Error,
        filepath: std::path::PathBuf,
        source_code: String,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match self {
            CheckEntry(error) => write!(f, "unable to check dir entry: {}", error),
            CreateDir(error) => write!(f, "unable to create dir: {}", error),
            ReadDir(error) => write!(f, "unable to read dir: {}", error),
            ReadEntry(error) => write!(f, "unable to read dir entry: {}", error),
            ReadFile(error) => write!(f, "unable to read file: {}", error),
            WriteFile(error) => write!(f, "unable to write file: {}", error),
            ParseFile {
                error,
                filepath,
                source_code,
            } => write!(
                f,
                "failed to parse file {:?}: {:?}\n\t{}",
                filepath, error, source_code
            ),
        }
    }
}

impl From<rules::dir_entry::CheckError> for Error {
    fn from(err: rules::dir_entry::CheckError) -> Self {
        Error::CheckEntry(err)
    }
}
