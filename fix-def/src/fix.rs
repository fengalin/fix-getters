//! Rust source file level getter definitions fixer.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use utils::{parser::prelude::*, Error, ParseFileError};

use crate::{GetterDefCollection, GetterDefVisitor};

/// Fixes the file at the given path.
///
/// If `output_path` is specified, the result will be written there,
/// otherwise the input files are overwritten.
pub fn fix(path: &Path, output_path: &Option<PathBuf>) -> Result<(), Error> {
    // Analyze Rust file
    let source_code =
        fs::read_to_string(path).map_err(|err| Error::ReadFile(path.to_owned(), err))?;
    let syntax_tree = match syn::parse_file(&source_code) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => {
            return Err(ParseFileError::new(error, path.to_owned(), source_code).into());
        }
    };

    let getter_collection = GetterDefCollection::default();
    GetterDefVisitor::visit(&path, &syntax_tree, &getter_collection);

    let output_path = match output_path {
        Some(output_path) => output_path,
        None => path,
    };

    if getter_collection.is_empty() {
        // Nothing to do for this file
        return Ok(());
    }

    // Write result
    let f = fs::File::create(output_path).map_err(|err| Error::WriteFile(path.to_owned(), err))?;
    let mut writer = std::io::BufWriter::new(f);

    for (line_idx, line) in source_code.lines().enumerate() {
        if let Some(getter_def) = getter_collection.get(line_idx) {
            if getter_def.needs_doc_alias() {
                writer
                    .write_fmt(format_args!("#[doc(alias = \"{}\")] ", getter_def.name()))
                    .map_err(|err| Error::WriteFile(path.to_owned(), err))?;
            }

            // Rename getter
            let origin = format!("fn {}(", getter_def.name());
            let target = format!("fn {}(", getter_def.new_name().as_str());

            writer
                .write(line.replacen(&origin, &target, 1).as_bytes())
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
