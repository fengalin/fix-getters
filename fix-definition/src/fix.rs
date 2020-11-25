use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use syn::visit::Visit;

use crate::{Error, GetterVisitor};

/// Fixes the file at the given path.
///
/// If `output_path` is specified, the result will be written there,
/// other the input files are overwitten.
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

    // Write result
    let f = fs::File::create(output_path).map_err(Error::WriteFile)?;
    {
        let mut writer = std::io::BufWriter::new(f);

        let mut renamable_getters = getter_visitor.renamable_getters.iter();
        let mut pending_rename = renamable_getters.next();
        for (line_nb, line) in source_code.lines().enumerate() {
            if let Some(pr) = pending_rename {
                if line_nb == pr.line_nb {
                    let origin = format!("fn {}(", pr.name);
                    let target = format!("fn {}(", pr.new_name);

                    // Add doc alias
                    if pr.needs_doc_alias {
                        writer
                            .write_fmt(format_args!("#[doc(alias = \"{}\")] ", pr.name))
                            .map_err(Error::WriteFile)?;
                    }

                    // Rename getter
                    writer
                        .write(line.replacen(&origin, &target, 1).as_bytes())
                        .map_err(Error::WriteFile)?;

                    pending_rename = renamable_getters.next();
                } else {
                    // Not the expected line => leave it unchanged
                    writer.write(line.as_bytes()).map_err(Error::WriteFile)?;
                }
            } else {
                // No more getters to rename
                writer.write(line.as_bytes()).map_err(Error::WriteFile)?;
            }

            writer.write(&[b'\n']).map_err(Error::WriteFile)?;
        }
    }

    Ok(())
}
