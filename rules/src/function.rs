//! Would-be-getter renaming rules definition.

use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::{self, Display},
};

/// Getters reserved suffix list.
///
/// Getter that we don't want to rename because
/// they are Rust keywords or would result confusing.
pub static RESERVED: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut reserved = HashSet::new();
    reserved.insert("");
    reserved.insert("as");
    reserved.insert("async");
    reserved.insert("await");
    reserved.insert("break");
    reserved.insert("const");
    reserved.insert("continue");
    reserved.insert("crate");
    reserved.insert("dyn");
    reserved.insert("else");
    reserved.insert("enum");
    reserved.insert("extern");
    reserved.insert("false");
    reserved.insert("fn");
    reserved.insert("for");
    reserved.insert("if");
    reserved.insert("impl");
    reserved.insert("in");
    reserved.insert("loop");
    reserved.insert("match");
    reserved.insert("mod");
    reserved.insert("move");
    reserved.insert("mut");
    reserved.insert("pub");
    reserved.insert("optional"); // keep `get_optional` similar to `get`
    reserved.insert("owned");
    reserved.insert("ref");
    reserved.insert("return");
    reserved.insert("self");
    reserved.insert("some"); // keep `get_some` similar to `get`
    reserved.insert("static");
    reserved.insert("struct");
    reserved.insert("super");
    reserved.insert("trait");
    reserved.insert("true");
    reserved.insert("type");
    reserved.insert("unchecked_mut"); // don't change call-sites: used in various Rust types
    reserved.insert("union");
    reserved.insert("unsafe");
    reserved.insert("use");
    reserved.insert("where");
    reserved.insert("while");
    reserved
});

/// Substitutes for getters returning a `bool`.
///
/// The convention is to rename `bool` getters `get_suffix` as `is_suffix`,
/// but there are cases for which we want a better name:
///
/// - `get_mute` -> `is_muted`.
/// - `get_emit_eos` -> `emits_eos`.
pub static BOOL_SUBSTITUTES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut bool_substitutes = HashMap::new();
    bool_substitutes.insert("activate", "activates");
    bool_substitutes.insert("accept", "accepts");
    bool_substitutes.insert("close", "closes");
    bool_substitutes.insert("create", "creates");
    bool_substitutes.insert("destroy", "destroys");
    bool_substitutes.insert("do", "does");
    bool_substitutes.insert("draw", "draws");
    bool_substitutes.insert("embed", "embeds");
    bool_substitutes.insert("emit", "emits");
    bool_substitutes.insert("enable", "enables");
    bool_substitutes.insert("exit", "exits");
    bool_substitutes.insert("expand", "expands");
    bool_substitutes.insert("fill", "fills");
    bool_substitutes.insert("fit", "fits");
    bool_substitutes.insert("focus", "focuses");
    bool_substitutes.insert("hide", "hides");
    bool_substitutes.insert("ignore", "ignores");
    bool_substitutes.insert("mute", "is_muted");
    bool_substitutes.insert("overwrite", "overwrites");
    bool_substitutes.insert("propagate", "propagates");
    bool_substitutes.insert("reset", "resets");
    bool_substitutes.insert("require", "requires");
    bool_substitutes.insert("resize", "resizes");
    bool_substitutes.insert("restrict", "restricts");
    bool_substitutes.insert("reveal", "reveals");
    bool_substitutes.insert("select", "selects");
    bool_substitutes.insert("show", "shows");
    bool_substitutes.insert("skip", "skips");
    bool_substitutes.insert("snap", "snaps");
    bool_substitutes.insert("support", "supports");
    bool_substitutes.insert("take", "takes");
    bool_substitutes.insert("use", "uses");
    bool_substitutes.insert("wrap", "wraps");
    bool_substitutes
});

/// Set of getters returning a `bool` for which the prefix should be removed.
///
/// The convention is to rename `bool` getters `get_suffix` as `is_suffix`,
/// but there are cases for which we want to apply the regular getter rule:
///
/// - `get_result` -> `result`.
/// - `get_has_entry` -> `has_entry`.
pub static BOOL_AS_REGULAR: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut bool_as_regular = HashSet::new();
    bool_as_regular.insert("can");
    bool_as_regular.insert("has");
    bool_as_regular.insert("must");
    bool_as_regular.insert("result");
    bool_as_regular.insert("should");
    bool_as_regular.insert("state");
    // Also add all the substitutes (e.g. accepts, skips, ...)
    for bool_substitute in BOOL_SUBSTITUTES.values() {
        bool_as_regular.insert(bool_substitute);
    }
    bool_as_regular
});

/// Getters prefix to move to the end.
///
/// The convention is to use the form:
/// - `get_structure_mut`.
///
/// but we can run into this one too:
/// - `get_mut_structure`.
pub static PREFIX_TO_POSTFIX: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut prefix_to_postfix = HashSet::new();
    prefix_to_postfix.insert("mut");
    prefix_to_postfix
});

/// Special suffix to detect getters returning a `bool`.
///
/// Ex.: `get_seekable`.
pub const BOOL_ABLE_PREFIX: &str = "able";

/// Attempts to apply getter name rules to this would-be-getter function.
///
/// The argument `returns_bool` hints the renaming process when
/// the getter returns a unique `bool` value. Use [`ReturnsBool::Maybe`]
/// if the return value is not known.
pub fn try_rename_would_be_getter(
    name: &str,
    returns_bool: impl Into<ReturnsBool>,
) -> Result<NewName, RenameError> {
    let suffix = match getter_suffix(name) {
        Some(suffix) => suffix,
        None => return Err(RenameError::NotGetFn),
    };

    try_rename_getter_suffix(suffix, returns_bool)
}

/// Attempts to apply getter name rules to this getter suffix.
///
/// The argument `returns_bool` hints the renaming process when
/// the getter returns a unique `bool` value. Use [`ReturnsBool::Maybe`]
/// if the return value is not known.
pub fn try_rename_getter_suffix(
    suffix: &str,
    returns_bool: impl Into<ReturnsBool>,
) -> Result<NewName, RenameError> {
    if RESERVED.contains(suffix) {
        return Err(RenameError::Reserved);
    }

    use ReturnsBool::*;
    match returns_bool.into() {
        False => (),
        True => return Ok(rename_bool_getter(suffix)),
        Maybe => {
            if let Some(rename) = guesstimate_boolness_then_rename(suffix) {
                return Ok(rename);
            }
        }
    }

    let splits: Vec<&str> = suffix.splitn(2, '_').collect();
    if splits.len() > 1 && PREFIX_TO_POSTFIX.contains(splits[0]) {
        Ok(NewName::Fixed(format!("{}_{}", splits[1], splits[0])))
    } else {
        Ok(NewName::Regular(suffix.to_string()))
    }
}

/// Retrieve the suffix from a would-be-getter function.
///
/// Returns `Some(*suffix*)` if name starts with `get_`.
#[inline]
pub fn getter_suffix(name: &str) -> Option<&str> {
    name.strip_prefix("get_")
}

/// Applies `bool` getter name rules.
#[inline]
pub fn rename_bool_getter(suffix: &str) -> NewName {
    if BOOL_AS_REGULAR.contains(suffix) {
        NewName::Fixed(suffix.to_string())
    } else if let Some(new_name) = try_substitute(suffix) {
        NewName::Substituted(new_name)
    } else {
        NewName::RegularBool(format!("is_{}", suffix))
    }
}

/// Attempts to apply special substitutions for boolean getters.
///
/// The substitutions are defined in [`BOOL_SUBSTITUTES`].
#[inline]
pub fn try_substitute(suffix: &str) -> Option<String> {
    let splits: Vec<&str> = suffix.splitn(2, '_').collect();
    BOOL_SUBSTITUTES.get(splits[0]).map(|substitute| {
        if splits.len() == 1 {
            substitute.to_string()
        } else {
            format!("{}_{}", substitute, splits[1])
        }
    })
}

/// Attempts to determine whether the getter returns a `bool` from its name.
///
/// Uses the [`BOOL_SUBSTITUTES`] and [`BOOL_ABLE_PREFIX`] as a best effort estimation.
///
/// Returns the name substitute if `self` seems to be returning a `bool`.
#[inline]
pub fn guesstimate_boolness_then_rename(suffix: &str) -> Option<NewName> {
    if let Some(new_name) = try_substitute(suffix) {
        return Some(NewName::Substituted(new_name));
    }

    let splits: Vec<&str> = suffix.splitn(2, '_').collect();
    if splits[0].ends_with(BOOL_ABLE_PREFIX) {
        Some(NewName::RegularBool(format!("is_{}", suffix)))
    } else {
        None
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ReturnsBool {
    True,
    False,
    Maybe,
}

impl ReturnsBool {
    pub fn is_true(&self) -> bool {
        matches!(self, ReturnsBool::True)
    }
}

impl From<bool> for ReturnsBool {
    fn from(returns_bool: bool) -> Self {
        if returns_bool {
            ReturnsBool::True
        } else {
            ReturnsBool::False
        }
    }
}

/// Would-be-getter rename attempt sucessful result.
///
/// Holds details about what happened.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum NewName {
    /// Fixed name to comply to rules. Ex. `get_active` -> `is_active`.
    Fixed(String),
    /// Applied substitution. Ex. `get_mute` -> `is_muted`.
    Substituted(String),
    /// Regular removal of the prefix.
    Regular(String),
    /// Replace `get_` with `is_`.
    RegularBool(String),
}

impl NewName {
    /// Returns the new name.
    pub fn as_str(&self) -> &str {
        use NewName::*;
        match self {
            Fixed(new_name) | Substituted(new_name) | Regular(new_name) | RegularBool(new_name) => {
                new_name.as_str()
            }
        }
    }

    /// Consumes the [`NewName`] and returns the inner new name [`String`].
    pub fn into_string(self) -> String {
        use NewName::*;
        match self {
            Fixed(new_name) | Substituted(new_name) | Regular(new_name) | RegularBool(new_name) => {
                new_name
            }
        }
    }

    /// Returns whether renaming required fixing the name to comply with rules.
    ///
    /// Ex. `get_active` -> `is_active`.
    pub fn is_fixed(&self) -> bool {
        matches!(self, NewName::Fixed(_))
    }

    /// Returns whether renaming required substituing (part) of the name.
    ///
    /// Ex. `get_mute` -> `is_muted`.
    pub fn is_substituted(&self) -> bool {
        matches!(self, NewName::Substituted(_))
    }

    /// Returns whether renaming used the regular strategy.
    ///
    /// Ex.:
    // * `get_name` -> `name`.
    // * `get_active` -> `is_active`.
    pub fn is_regular(&self) -> bool {
        matches!(self, NewName::Regular(_) | NewName::RegularBool(_))
    }

    /// Returns whether renaming used the regular strategy for booleans.
    ///
    /// Ex. `get_active` -> `is_active`.
    pub fn is_regular_bool(&self) -> bool {
        matches!(self, NewName::RegularBool(_))
    }
}

impl Display for NewName {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use NewName::*;
        match self {
            Fixed(new_name) => write!(f, "fixed as {}", new_name),
            Substituted(new_name) => write!(f, "substituted with {}", new_name),
            Regular(new_name) => write!(f, "renamed as {}", new_name),
            RegularBool(new_name) => write!(f, "renamed as {}", new_name),
        }
    }
}

impl<T: AsRef<str>> PartialEq<T> for NewName {
    fn eq(&self, other: &T) -> bool {
        self.as_str() == other.as_ref()
    }
}

/// Would-be-getter rename attempt failure result.
///
/// Holds details about the reason of the failure.
#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum RenameError {
    /// The function doesn't start with `get_`.
    NotGetFn,
    /// The function uses a reserved name and can't be renamed.
    Reserved,
}

impl RenameError {
    pub fn is_not_get_fn(&self) -> bool {
        matches!(self, RenameError::NotGetFn)
    }

    pub fn is_reserved(&self) -> bool {
        matches!(self, RenameError::Reserved)
    }
}

impl Display for RenameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use RenameError::*;
        match self {
            NotGetFn => write!(f, "not a get function"),
            Reserved => write!(f, "name is reserved"),
        }
    }
}

impl Error for RenameError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substitution() {
        assert_eq!(try_substitute(&"mute").unwrap(), "is_muted");
        assert_eq!(try_substitute(&"emit_eos").unwrap(), "emits_eos");
    }

    #[test]
    fn bool_getter_suffix() {
        let new_name = rename_bool_getter(&"activable");
        assert!(new_name.is_regular());
        assert!(new_name.is_regular_bool());
        assert_eq!(new_name, "is_activable");

        let new_name = rename_bool_getter(&"mute");
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "is_muted");

        let new_name = rename_bool_getter(&"emit_eos");
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "emits_eos");

        let new_name = rename_bool_getter(&"result");
        assert!(new_name.is_fixed());
        assert_eq!(new_name, "result");
    }

    #[test]
    fn boolness_guestimation() {
        assert!(guesstimate_boolness_then_rename(&"result").is_none());

        let new_name = guesstimate_boolness_then_rename(&"mute").unwrap();
        assert_eq!(new_name, "is_muted");

        let new_name = guesstimate_boolness_then_rename(&"activable").unwrap();
        assert_eq!(new_name, "is_activable");
    }

    #[test]
    fn rename_getter_non_bool() {
        let new_name = try_rename_would_be_getter(&"get_structure", false).unwrap();
        assert!(new_name.is_regular());
        assert_eq!(new_name, "structure");

        // Bool-alike, but not a bool
        let new_name = try_rename_would_be_getter(&"get_activable", false).unwrap();
        assert!(new_name.is_regular());
        assert_eq!(new_name, "activable");

        // Prefix to postfix
        let new_name = try_rename_would_be_getter(&"get_mut_structure", false).unwrap();
        assert!(new_name.is_fixed());
        assert_eq!(new_name, "structure_mut");

        assert!(try_rename_would_be_getter(&"get_mut", false)
            .unwrap_err()
            .is_reserved());
        assert!(try_rename_would_be_getter(&"not_a_getter", false)
            .unwrap_err()
            .is_not_get_fn());
    }

    #[test]
    fn rename_getter_bool() {
        let new_name = try_rename_would_be_getter(&"get_structure", true).unwrap();
        assert!(new_name.is_regular_bool());
        assert_eq!(new_name, "is_structure");

        let new_name = try_rename_would_be_getter(&"get_mute", true).unwrap();
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "is_muted");

        let new_name = try_rename_would_be_getter(&"get_emit_eos", true).unwrap();
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_would_be_getter(&"get_activable", true).unwrap();
        assert!(new_name.is_regular_bool());
        assert_eq!(new_name, "is_activable");

        assert!(try_rename_would_be_getter(&"get_mut", true)
            .unwrap_err()
            .is_reserved());
        assert!(try_rename_would_be_getter(&"not_a_getter", true)
            .unwrap_err()
            .is_not_get_fn());
    }

    #[test]
    fn rename_getter_maybe_bool() {
        let new_name = try_rename_would_be_getter(&"get_structure", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_regular());
        assert_eq!(new_name, "structure");

        let new_name = try_rename_would_be_getter(&"get_mute", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "is_muted");

        let new_name = try_rename_would_be_getter(&"get_emit_eos", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_would_be_getter(&"get_activable", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_regular_bool());
        assert_eq!(new_name, "is_activable");

        assert!(try_rename_would_be_getter(&"get_mut", ReturnsBool::Maybe)
            .unwrap_err()
            .is_reserved());
        assert!(
            try_rename_would_be_getter(&"not_a_getter", ReturnsBool::Maybe)
                .unwrap_err()
                .is_not_get_fn()
        );
    }
}
