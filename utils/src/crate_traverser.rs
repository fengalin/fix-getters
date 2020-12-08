//! Filesystem traversal.

#[cfg(feature = "log")]
use log::debug;
use rules::dir_entry;
use std::path::{Path, PathBuf};

use crate::Error;

/// Traverses the file system from the given path.
///
/// The dir entries are traversed according to the rules defined
/// in [`rules::dir_entry`]. When the rules indicate that a file
/// must be processed, the `f` function is called with the appropriate
/// `path` and `output_path`.
///
/// If `output_path` is specified, the traversed tree is replicated there.
pub fn traverse<F>(path: &Path, output_path: &Option<PathBuf>, f: &F) -> Result<(), Error>
where
    F: Fn(&Path, &Option<PathBuf>) -> Result<(), Error>,
{
    // Traverse the crate / workspace tree
    if path.is_dir() {
        #[cfg(feature = "log")]
        debug!("entering {:?}", path);

        for entry in std::fs::read_dir(path).map_err(|err| Error::ReadDir(path.to_owned(), err))? {
            let entry = entry.map_err(|err| Error::ReadEntry(path.to_owned(), err))?;

            use dir_entry::CheckOk::*;
            let is_dir = match dir_entry::check(&entry)? {
                Directory => true,
                RustFile => false,
                Skip(_name) => {
                    #[cfg(feature = "log")]
                    debug!("skipping {:?}", _name);
                    continue;
                }
                SkipUnspecified => continue,
            };

            let path = entry.path();
            let output_path = match output_path.as_ref() {
                Some(output_path) => {
                    let output_path = output_path.join(entry.file_name());
                    if is_dir {
                        std::fs::create_dir(&output_path)
                            .map_err(|err| Error::CreateDir(output_path.to_owned(), err))?;
                    }
                    Some(output_path)
                }
                None => None,
            };

            traverse(&path, &output_path, f)?;
        }

        return Ok(());
    }

    #[cfg(feature = "log")]
    debug!("processing {:?}", path);
    f(&path, &output_path)
}
