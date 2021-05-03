mod doc_alias_mode;
pub use doc_alias_mode::DocAliasMode;

mod fixer;
use fixer::GetterDefFixer;

mod getter_def;
pub use getter_def::GetterDef;

mod collectors;
pub use collectors::*;

use log::{error, info};
use std::{path::PathBuf, process};
use utils::prelude::*;

fn main() {
    let m = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            clap::Arg::with_name("no-doc-aliases")
                .short("n")
                .long("no-doc-aliases")
                .help("Don't had doc aliases to the renamed functions"),
        )
        // Deprecated since 0.3.1.
        // FIXME remove in next major version.
        .arg(
            clap::Arg::with_name("doc-alias")
                .short("d")
                .long("doc-alias")
                .help("Deprecated. This is the default. Had a doc alias to the renamed functions"),
        )
        .arg(
            clap::Arg::with_name("conservative")
                .short("c")
                .long("conservative")
                .help("Be conservative when selecting getter functions"),
        )
        .arg(
            clap::Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Run silently"),
        )
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Show detailed logs"),
        )
        .arg(clap::Arg::with_name("PATH").help("Crate or workspace root path"))
        .arg(clap::Arg::with_name("OUTPUT").help("Output to a different root path"))
        .get_matches();

    stderrlog::new()
        .verbosity(if m.is_present("verbose") {
            5
        } else if m.is_present("quiet") {
            1
        } else {
            2
        })
        .init()
        .unwrap();

    let path: PathBuf = match m.value_of("PATH") {
        Some(path) => path.into(),
        None => PathBuf::from("."),
    };

    if !path.exists() {
        error!(
            "path not found {}",
            path.to_str().expect("was a &str initially")
        );
        process::exit(1);
    }

    let output_path: Option<PathBuf> = if let Some(output) = m.value_of("OUTPUT") {
        let output_path: PathBuf = output.to_string().into();
        if !output_path.exists() {
            error!(
                "output path not found {}",
                output_path.to_str().expect("was a &str initially")
            );
            process::exit(1);
        }
        Some(output_path)
    } else {
        None
    };

    // Traverse the given crate tree following the rules defined in crate `rules`
    // and fix the elligible files.
    let mut fixer = GetterDefFixer::new(
        if m.is_present("conservative") {
            IdentificationMode::Conservative
        } else {
            IdentificationMode::AllGetFunctions
        },
        if m.is_present("no-doc-aliases") {
            DocAliasMode::Discard
        } else {
            DocAliasMode::Generate
        },
    );
    info!("Processing {:?}", path);
    if let Err(error) = fixer.traverse(&path, &output_path) {
        let _ = error!("{}", error);
        process::exit(1);
    }
    info!("Done {:?}", path);
}
