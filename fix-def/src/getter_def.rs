//! A [`Getter`](utils::Getter) definition.

use rules::{NewName, ReturnsBool};
use std::{
    fmt::{self, Display},
    path::Path,
};
use utils::{Getter, GetterError};

/// A [`Getter`](utils::Getter) definition.
#[derive(Debug)]
pub struct GetterDef {
    getter: Getter,
    needs_doc_alias: bool,
}

impl GetterDef {
    pub fn try_new(
        name: String,
        returns_bool: impl Into<ReturnsBool> + Copy,
        line: usize,
    ) -> Result<Self, GetterError> {
        Getter::try_new(name, returns_bool, line).map(|getter| GetterDef {
            getter,
            needs_doc_alias: false,
        })
    }

    pub fn name(&self) -> &str {
        &self.getter.name
    }

    pub fn new_name(&self) -> &NewName {
        &self.getter.new_name
    }

    pub fn returns_bool(&self) -> ReturnsBool {
        self.getter.new_name.returns_bool()
    }

    pub fn set_returns_bool(&mut self, returns_bool: impl Into<ReturnsBool>) {
        self.getter.set_returns_bool(returns_bool);
    }

    pub fn line(&self) -> usize {
        self.getter.line
    }

    pub fn needs_doc_alias(&self) -> bool {
        self.needs_doc_alias
    }

    pub fn set_needs_doc_alias(&mut self, needs_doc_alias: bool) {
        self.needs_doc_alias = needs_doc_alias;
    }

    pub fn log(&self, path: &Path, scope: &dyn Display) {
        self.getter.log(path, scope);
    }
}

impl Display for GetterDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.getter,
            if self.needs_doc_alias {
                " needs doc alias"
            } else {
                ""
            },
        )
    }
}
