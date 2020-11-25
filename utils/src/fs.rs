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
        debug!("Crate tree traversal entering {:?}", path.as_os_str());

        for entry in std::fs::read_dir(path).map_err(Error::ReadDir)? {
            let entry = entry.map_err(Error::ReadEntry)?;

            use dir_entry::CheckOk::*;
            let is_dir = match dir_entry::check(&entry)? {
                Directory => true,
                RsFile => false,
                Skip(name) => {
                    debug!("Crate tree traversal skipping {:?}", name);
                    continue;
                }
                SkipUnspecified => continue,
            };

            let path = entry.path();
            let output_path = match output_path.as_ref() {
                Some(output_path) => {
                    let output_path = output_path.join(entry.file_name());
                    if is_dir {
                        std::fs::create_dir(&output_path).map_err(Error::CreateDir)?;
                    }
                    Some(output_path)
                }
                None => None,
            };

            traverse(&path, &output_path, f)?;
        }

        return Ok(());
    }

    debug!("Crate tree traversal processing {:?}", path.as_os_str());
    f(&path, &output_path)
}
