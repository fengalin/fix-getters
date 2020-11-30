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

impl Display for GetterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "skipping {}() {} @ {}", self.name, self.err, self.line)
    }
}

impl Error for GetterError {}

impl Getter {
    /// Attempts to build a `Getter` from the provided data.
    pub fn try_new(
        name: String,
        returns_bool: ReturnsBool,
        line: usize,
    ) -> Result<Self, GetterError> {
        match rules::try_rename_getter(&name, returns_bool) {
            Ok(new_name) => Ok(Getter {
                name,
                new_name,
                line,
                returns_bool,
            }),
            Err(err) => Err(GetterError { name, err, line }),
        }
    }

    /// Attempts to build a `Getter` from the provided data and log the result.
    #[cfg(feature = "log")]
    pub fn try_new_and_log(
        scope: &dyn Display,
        name: String,
        returns_bool: ReturnsBool,
        line: usize,
    ) -> Result<Self, GetterError> {
        match Self::try_new(name, returns_bool, line) {
            Ok(getter) => {
                getter.log(scope);
                Ok(getter)
            }
            Err(err) => {
                log_err(scope, &err);
                Err(err)
            }
        }
    }

    /// Logs details about the getter at the appropriate log level.
    #[cfg(feature = "log")]
    pub fn log(&self, scope: &dyn Display) {
        if self.new_name.is_fixed() {
            debug!("* {}: {}", scope, self);
        } else if self.new_name.is_substituted() {
            warn!("* {}: {}", scope, self);
        } else {
            trace!("* {}: {}", scope, self);
        }
    }
}

impl Display for Getter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ReturnsBool::*;
        let return_str = match self.returns_bool {
            False | Maybe => "",
            True => " -> bool",
        };
        write!(
            f,
            "{}(){} {}() @ {}",
            self.name, return_str, self.new_name, self.line
        )
    }
}

/// Logs the failed attempt at renaming a would be getter.
#[cfg(feature = "log")]
pub fn log_err(scope: &dyn Display, err: &GetterError) {
    if !err.err.is_not_get_fn() {
        debug!("* {}: {}", scope, err);
    } else {
        trace!("* {}: {}", scope, err);
    }
}

/// Logs the reason for skipping the `name` function.
#[cfg(feature = "log")]
pub fn skip(scope: &dyn Display, name: String, reason: &dyn Display, line: usize) {
    debug!("* {}: skipping {}() {} @ {}", scope, name, reason, line);
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
