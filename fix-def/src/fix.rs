//! Rust source file level getter definitions fixer.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use syn::visit::Visit;

use utils::{Error, ParseFileError};

use crate::GetterDefsVisitor;

/// Fixes the file at the given path.
///
/// If `output_path` is specified, the result will be written there,
/// otherwise the input files are overwritten.
pub(crate) fn fix(path: &Path, output_path: &Option<PathBuf>) -> Result<(), Error> {
    // Analyze Rust file
    let source_code = fs::read_to_string(path).map_err(Error::ReadFile)?;
    let syntax_tree = match syn::parse_file(&source_code) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => {
            return Err(ParseFileError::new(error, path.to_owned(), source_code).into());
        }
    };

    let mut visitor = GetterDefsVisitor::default();
    visitor.visit_file(&syntax_tree);

    let output_path = match output_path {
        Some(output_path) => output_path,
        None => path,
    };

    if visitor.getter_defs.is_empty() {
        // Nothing to do for this file
        return Ok(());
    }

    // Write result
    let f = fs::File::create(output_path).map_err(Error::WriteFile)?;
    let mut writer = std::io::BufWriter::new(f);

    for (line_idx, line) in source_code.lines().enumerate() {
        if let Some(getter_defs) = visitor.getter_defs.get(&line_idx) {
            if getter_defs.needs_doc_alias() {
                writer
                    .write_fmt(format_args!("#[doc(alias = \"{}\")] ", getter_defs.name()))
                    .map_err(Error::WriteFile)?;
            }

            // Rename getter
            let origin = format!("fn {}(", getter_defs.name());
            let target = format!("fn {}(", getter_defs.new_name().as_str());

            writer
                .write(line.replacen(&origin, &target, 1).as_bytes())
                .map_err(Error::WriteFile)?;
        } else {
            // No changes for this line
            writer.write(line.as_bytes()).map_err(Error::WriteFile)?;
        }

        writer.write(&[b'\n']).map_err(Error::WriteFile)?;
    }

    Ok(())
}
