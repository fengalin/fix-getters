use lazy_static::lazy_static;
use std::collections::HashSet;
use std::fs::DirEntry;
use std::io;
use std::{
    error::Error,
    fmt::{self, Display},
};

lazy_static! {
    /// Directories to exclude from the fix process.
    pub static ref EXCLUDED: HashSet<&'static str> ={
        let mut excluded = HashSet::new();
        excluded.insert(".git");
        excluded.insert("auto");
        excluded.insert("ci");
        excluded.insert("docs");
        excluded.insert("gir");
        excluded.insert("gir-files");
        excluded.insert("target");
        excluded.insert("sys");
        excluded
    };
}

/// Checks the given directory entry.
#[inline]
pub fn check(entry: &DirEntry) -> Result<CheckOk, CheckError> {
    let entry_type = entry.file_type().map_err(CheckError::DirEntry)?;

    let entry_name = entry.file_name();
    let entry_name = match entry_name.to_str() {
        Some(entry_name) => entry_name,
        None => return Err(CheckError::Name(entry_name)),
    };

    if entry_type.is_file() {
        if entry_name.ends_with(".rs") {
            return Ok(CheckOk::RsFile);
        }
    } else if entry_type.is_dir() {
        if !EXCLUDED.contains(entry_name) {
            return Ok(CheckOk::Directory);
        }
    } else {
        return Ok(CheckOk::SkipUnspecified);
    }

    Ok(CheckOk::Skip(entry_name.to_string()))
}

#[derive(Debug)]
pub enum CheckOk {
    Directory,
    RsFile,
    Skip(String),
    SkipUnspecified,
}

#[derive(Debug)]
pub enum CheckError {
    Name(std::ffi::OsString),
    DirEntry(io::Error),
}

impl Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use CheckError::*;

        match self {
            Name(name) => write!(f, "error converting dir entry name {:?}", name),
            DirEntry(err) => write!(f, "error checking dir entry {}", err),
        }
    }
}

impl Error for CheckError {}
