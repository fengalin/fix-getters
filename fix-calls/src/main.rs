mod fixer;
use fixer::GetterCallFixer;

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
        .arg(
            clap::Arg::with_name("PATH")
                .help("Crate or workspace root path (default: current directory)"),
        )
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
    // and apply `fix` on elligible files.
    let mut fixer = GetterCallFixer::new(if m.is_present("conservative") {
        IdentificationMode::Conservative
    } else {
        IdentificationMode::AllGetFunctions
    });
    info!("Processing {:?}", path);
    if let Err(error) = fixer.traverse(&path, &output_path) {
        let _ = error!("{}", error);
        process::exit(1);
    }
    info!("Done {:?}", path);
}
