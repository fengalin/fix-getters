//! Directory entry filtering.

use once_cell::sync::Lazy;
use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Display},
    fs::DirEntry,
    io,
    path::PathBuf,
};

/// Directories to exclude from the fix process.
pub static EXCLUDED: Lazy<HashSet<&'static str>> = Lazy::new(|| {
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
});

/// Checks the given directory entry.
#[inline]
pub fn check(entry: &DirEntry) -> Result<CheckOk, CheckError> {
    let entry_type = entry
        .file_type()
        .map_err(|err| CheckError::DirEntry(entry.path(), err))?;

    let entry_name = entry.file_name();
    let entry_name = match entry_name.to_str() {
        Some(entry_name) => entry_name,
        None => return Err(CheckError::Name(entry.path(), entry_name)),
    };

    if entry_type.is_file() {
        if entry_name.ends_with(".rs") {
            return Ok(CheckOk::RustFile);
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
    RustFile,
    Skip(String),
    SkipUnspecified,
}

#[derive(Debug)]
pub enum CheckError {
    Name(PathBuf, std::ffi::OsString),
    DirEntry(PathBuf, io::Error),
}

impl Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use CheckError::*;

        match self {
            Name(path, name) => write!(f, "error converting dir entry name {:?} {:?}", path, name),
            DirEntry(path, err) => write!(f, "error checking dir entry {:?}: {}", path, err),
        }
    }
}

impl Error for CheckError {}
