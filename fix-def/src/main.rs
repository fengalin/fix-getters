mod fix;
use fix::fix;

mod getter_visitor;
pub(crate) use getter_visitor::GetterDefsVisitor;

pub(crate) mod macro_parser;

use log::{error, info};
use std::{
    fmt::{self, Display},
    path::PathBuf,
    process,
};
use utils::{fs, Getter, GetterError};

use rules::{NewName, ReturnsBool};

#[derive(Debug)]
struct GetterDef {
    name: String,
    new_name: NewName,
    returns_bool: ReturnsBool,
    line: usize,
    needs_doc_alias: bool,
}

impl GetterDef {
    fn try_new(
        name: String,
        returns_bool: impl Into<ReturnsBool> + Copy,
        line: usize,
        needs_doc_alias: bool,
    ) -> Result<Self, GetterError> {
        Getter::try_new(name, returns_bool, line)
            .map(|getter| GetterDef::from(getter, needs_doc_alias))
    }

    fn try_new_and_log(
        scope: &dyn Display,
        name: String,
        returns_bool: impl Into<ReturnsBool> + Copy,
        line: usize,
        needs_doc_alias: bool,
    ) -> Result<Self, GetterError> {
        Getter::try_new_and_log(scope, name, returns_bool, line)
            .map(|getter| GetterDef::from(getter, needs_doc_alias))
    }

    fn from(getter: Getter, needs_doc_alias: bool) -> Self {
        GetterDef {
            name: getter.name,
            new_name: getter.new_name,
            line: getter.line,
            needs_doc_alias,
            returns_bool: getter.returns_bool,
        }
    }
}

impl Display for GetterDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ReturnsBool::*;
        let return_str = match self.returns_bool {
            False => "",
            True => " -> bool",
            Maybe => " -> maybe bool",
        };
        write!(
            f,
            "{}(){} {}() @ {}",
            self.name, return_str, self.new_name, self.line
        )
    }
}

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
