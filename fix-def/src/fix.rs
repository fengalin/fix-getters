use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use syn::visit::Visit;

use crate::{Error, GetterVisitor};

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
            return Err(Error::ParseFile {
                error,
                filepath: path.to_owned(),
                source_code,
            });
        }
    };

    let mut getter_visitor = GetterVisitor::default();
    getter_visitor.visit_file(&syntax_tree);

    let output_path = match output_path {
        Some(output_path) => output_path,
        None => path,
    };

    if getter_visitor.renamable_lines.is_empty() {
        // Nothing to do for this file
        return Ok(());
    }

    // Write result
    let f = fs::File::create(output_path).map_err(Error::WriteFile)?;
    let mut writer = std::io::BufWriter::new(f);

    for (line_idx, line) in source_code.lines().enumerate() {
        if let Some(rd) = getter_visitor.renamable_lines.get(&line_idx) {
            if rd.needs_doc_alias {
                writer
                    .write_fmt(format_args!("#[doc(alias = \"{}\")] ", rd.rename.name()))
                    .map_err(Error::WriteFile)?;
            }

            // Rename getter
            let origin = format!("fn {}(", rd.rename.name());
            let target = format!("fn {}(", rd.rename.new_name());

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
