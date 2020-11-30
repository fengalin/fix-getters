//! `Getter` helper.

#[cfg(feature = "log")]
use log::{debug, trace, warn};

use std::{
    error::Error,
    fmt::{self, Display},
};

use rules::{self, NewName, RenameError, ReturnsBool};

/// `Getter` helper.
///
/// A `Getter` is a function for which the renaming rules defined in crate
/// [`fix-getters-rules`](../../fix_getters_rules/function/index.html) hold.
#[derive(Debug)]
pub struct Getter {
    pub name: String,
    pub new_name: NewName,
    pub returns_bool: ReturnsBool,
    pub line: usize,
}

#[derive(Debug)]
pub struct GetterError {
    pub name: String,
    pub err: RenameError,
    pub line: usize,
}

impl GetterError {
    /// Logs details about the getter creation failure at the appropriate log level.
    #[cfg(feature = "log")]
    pub fn log(&self, scope: &dyn Display) {
        if !self.err.is_not_get_fn() {
            debug!("* {} {}", scope, self);
        } else {
            trace!("* {} {}", scope, self);
        }
    }
}

impl Display for GetterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@ {}: skipping {}() {}", self.line, self.name, self.err)
    }
}

impl Error for GetterError {}

impl Getter {
    /// Attempts to build a `Getter` from the provided data.
    pub fn try_new(
        name: String,
        returns_bool: impl Into<ReturnsBool> + Copy,
        line: usize,
    ) -> Result<Self, GetterError> {
        match rules::try_rename_getter(&name, returns_bool) {
            Ok(new_name) => Ok(Getter {
                name,
                new_name,
                line,
                returns_bool: returns_bool.into(),
            }),
            Err(err) => Err(GetterError { name, err, line }),
        }
    }

    /// Logs details about the getter at the appropriate log level.
    #[cfg(feature = "log")]
    pub fn log(&self, scope: &dyn Display) {
        if self.new_name.is_fixed() {
            debug!("* {} {}", scope, self);
        } else if self.new_name.is_substituted() {
            warn!("* {} {}", scope, self);
        } else {
            trace!("* {} {}", scope, self);
        }
    }
}

impl Display for Getter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ReturnsBool::*;
        let return_str = match self.returns_bool {
            False => "",
            True => " -> bool",
            Maybe => " -> ?",
        };
        write!(
            f,
            "@ {}: {}(){} {}()",
            self.line, self.name, return_str, self.new_name,
        )
    }
}

/// Logs the reason for skipping the `name` function.
#[cfg(feature = "log")]
pub fn skip(scope: &dyn Display, name: &str, reason: &dyn Display, line: usize) {
    debug!("* {} @ {}: skipping {}() {}", scope, line, name, reason);
}

/// Reason for considering a function is not a getter.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum NonGetterReason {
    GenericTypeParam,
    MultipleArgs,
    NotAGet,
    NotAMethod,
    NonSelfUniqueArg,
    NoArgs,
}

impl Display for NonGetterReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NonGetterReason::*;
        match self {
            GenericTypeParam => f.write_str("generic type parameter(s)"),
            MultipleArgs => f.write_str("multiple arguments (incl. self)"),
            NotAGet => f.write_str("not a get function"),
            NotAMethod => f.write_str("not a method"),
            NonSelfUniqueArg => f.write_str("unique argument is not self"),
            NoArgs => f.write_str("no arguments"),
        }
    }
}
