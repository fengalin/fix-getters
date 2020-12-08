//! Crate traversal mechanism.

#[cfg(feature = "log")]
use log::debug;
use rules::dir_entry;
use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

use crate::Error;

/// Crate traversal mechanism.
///
/// By default, the dir entries are traversed according to the rules defined
/// in [`rules::dir_entry`].
///
/// If `output_path` is specified, the traversed tree is replicated there.
pub trait CrateTraverser {
    /// Called when the path points to a Rust file.
    fn handle_rust_file(&mut self, path: &Path, output_path: &Option<PathBuf>)
        -> Result<(), Error>;

    fn handle_skipped_dir_entry(
        &mut self,
        _entry: &DirEntry,
        _output_path: &Option<PathBuf>,
    ) -> Result<(), Error> {
        Ok(())
    }

    /// Traverses the crate or workspace from the specified path.
    fn traverse(&mut self, path: &Path, output_path: &Option<PathBuf>) -> Result<(), Error> {
        if path.is_dir() {
            #[cfg(feature = "log")]
            debug!("entering {:?}", path);

            for entry in
                std::fs::read_dir(path).map_err(|err| Error::ReadDir(path.to_owned(), err))?
            {
                let entry = entry.map_err(|err| Error::ReadEntry(path.to_owned(), err))?;

                use dir_entry::CheckOk::*;
                let is_dir = match dir_entry::check(&entry)? {
                    Directory => true,
                    RustFile => false,
                    Skip(_) => {
                        #[cfg(feature = "log")]
                        debug!("skipping {:?}", entry.file_name().to_str());
                        self.handle_skipped_dir_entry(&entry, output_path)?;
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

                self.traverse(&path, &output_path)?;
            }

            return Ok(());
        }

        #[cfg(feature = "log")]
        debug!("processing {:?}", path);
        self.handle_rust_file(path, output_path)
    }
}
