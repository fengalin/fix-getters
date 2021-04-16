//! Rust source file level getter calls fixer.

use std::borrow::Cow;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use utils::{prelude::*, Error, ParseFileError};

use crate::{GetterCallCollection, StGetterCallCollector};

/// Rust source file level getter calls fixer.
pub struct GetterCallFixer {
    identification_mode: IdentificationMode,
}

impl GetterCallFixer {
    pub fn new(identification_mode: IdentificationMode) -> Self {
        GetterCallFixer {
            identification_mode,
        }
    }
}

impl CrateTraverser for GetterCallFixer {
    /// Fixes the file at the given path.
    ///
    /// If `output_path` is specified, the result will be written there,
    /// otherwise the input files are overwritten.
    fn handle_rust_file(
        &mut self,
        path: &Path,
        output_path: &Option<PathBuf>,
    ) -> Result<(), Error> {
        // Analyze Rust file
        let source_code =
            fs::read_to_string(path).map_err(|err| Error::ReadFile(path.to_owned(), err))?;
        let syntax_tree = match syn::parse_file(&source_code) {
            Ok(syntax_tree) => syntax_tree,
            Err(error) => {
                return Err(ParseFileError::new(error, path.to_owned(), source_code).into());
            }
        };

        let getter_collection = GetterCallCollection::default();
        StGetterCallCollector::collect(
            path,
            &syntax_tree,
            self.identification_mode,
            &getter_collection,
        );

        let output_path = match output_path {
            Some(output_path) => output_path,
            None => path,
        };

        if getter_collection.is_empty() {
            // Nothing to do for this file
            return Ok(());
        }

        // Write result
        let f =
            fs::File::create(output_path).map_err(|err| Error::WriteFile(path.to_owned(), err))?;
        let mut writer = std::io::BufWriter::new(f);

        for (line_idx, line) in source_code.lines().enumerate() {
            if let Some(getter_calls) = getter_collection.get(line_idx) {
                let mut line = Cow::from(line);
                for getter_call in getter_calls {
                    line = Cow::from(line.replacen(
                        &getter_call.name,
                        getter_call.new_name.as_str(),
                        1,
                    ));
                }
                writer
                    .write(line.as_bytes())
                    .map_err(|err| Error::WriteFile(path.to_owned(), err))?;
            } else {
                // No changes for this line
                writer
                    .write(line.as_bytes())
                    .map_err(|err| Error::WriteFile(path.to_owned(), err))?;
            }

            writer
                .write(&[b'\n'])
                .map_err(|err| Error::WriteFile(path.to_owned(), err))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn fix_baseline(id_mode: IdentificationMode) {
        let input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test_samples")
            .join("input");

        let file_id = if id_mode.is_conservative() {
            "conservative"
        } else {
            "all_get_functions"
        };

        let output_path = env::temp_dir().join("fix-calls").join(file_id);
        fs::create_dir_all(&output_path).unwrap();
        let output_file = output_path.clone().join("baseline.rs");

        let mut fixer = GetterCallFixer::new(id_mode);
        fixer.traverse(&input_path, &Some(output_path)).unwrap();

        let output = fs::read_to_string(&output_file).unwrap();

        // Uncomment to keep output
        /*
        let keep_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test_samples")
            .join(format!("output_{}.rs", file_id));
        fs::copy(output_file, keep_path).unwrap();
        */

        let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test_samples")
            .join("expected")
            .join(format!("{}.rs", file_id));
        let expected = fs::read_to_string(&expected_path).unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn fix_baseline_conservative() {
        fix_baseline(IdentificationMode::Conservative)
    }

    #[test]
    fn fix_baseline_all_get_functions() {
        fix_baseline(IdentificationMode::AllGetFunctions)
    }
}
