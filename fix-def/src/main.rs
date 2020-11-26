mod fix;
use fix::fix;

mod getter_visitor;
pub(crate) use getter_visitor::GetterVisitor;

use log::{error, info};
use std::path::PathBuf;
use std::process;
use utils::{fs, Error};

fn main() {
    let m = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
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
                .required(true)
                .help("Crate or workspace root path"),
        )
        .arg(clap::Arg::with_name("OUTPUT").help("Output to a different root path"))
        .get_matches();

    stderrlog::new()
        .verbosity(if m.is_present("verbose") {
            4
        } else {
            if m.is_present("quiet") {
                1
            } else {
                2
            }
        })
        .init()
        .unwrap();

    let path: PathBuf = m
        .value_of("PATH")
        .expect("checked by claps")
        .to_string()
        .into();

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
    info!(
        "Processing {}",
        path.to_str().expect("was a &str initially"),
    );
    if let Err(error) = fs::traverse(&path, &output_path, &fix) {
        let _ = error!("{}", error);
        process::exit(1);
    }
    info!("Done {}", path.to_str().expect("was a &str initially"));
}
