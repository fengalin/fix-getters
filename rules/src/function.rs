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

/// Subsitutes of `bool` getter to be used when the suffixes matches exactly.
///
/// The convention is to rename `bool` getters `get_suffix` as `is_suffix`,
/// but there are cases for which it would be confusing to add the `is` prefix:
///
/// - `get_result` -> `result`.
/// - `get_overwrite` -> `overwrites`. Note that if the getter suffix doesn't match
///   exactly, this rule doesn't apply. Ex. `get_overwrite_mode` -> `is_overwrites_mode`
pub static BOOL_EXACT_SUBSTITUTES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut exact_substitutes = HashMap::new();
    exact_substitutes.insert("result", "result");
    exact_substitutes.insert("overwrite", "overwrites");
    exact_substitutes.insert("visibility", "is_visible");
    exact_substitutes
});

/// Substitutes for getters returning a `bool`.
///
/// The convention is to rename `bool` getters `get_suffix` as `is_suffix`,
/// but there are cases for which we want a better name:
///
/// - `get_mute` -> `is_muted`.
/// - `get_emit_eos` -> `emits_eos`.
pub static BOOL_PREFIX_SUBSTITUTES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut prefix_substitutes = HashMap::new();
    prefix_substitutes.insert("activate", "activates");
    prefix_substitutes.insert("accept", "accepts");
    prefix_substitutes.insert("allow", "allows");
    // Ex.: `get_always_show_image` -> `must_always_show_image`.
    prefix_substitutes.insert("always", "must_always");
    prefix_substitutes.insert("close", "closes");
    prefix_substitutes.insert("create", "creates");
    // Ex.: `get_destroy_with_parent` -> `must_destroy_with_parent`.
    prefix_substitutes.insert("destroy", "must_destroy");
    prefix_substitutes.insert("do", "does");
    prefix_substitutes.insert("draw", "draws");
    prefix_substitutes.insert("embed", "embeds");
    prefix_substitutes.insert("emit", "emits");
    prefix_substitutes.insert("enable", "enables");
    prefix_substitutes.insert("exit", "exits");
    prefix_substitutes.insert("expand", "expands");
    prefix_substitutes.insert("fill", "fills");
    prefix_substitutes.insert("fit", "fits");
    // Ex.: `get_focus_on_map` -> `gets_focus_on_map`.
    prefix_substitutes.insert("focus", "gets_focus");
    prefix_substitutes.insert("follow", "follows");
    prefix_substitutes.insert("hide", "hides");
    prefix_substitutes.insert("ignore", "ignores");
    prefix_substitutes.insert("invert", "inverts");
    prefix_substitutes.insert("mute", "is_muted");
    prefix_substitutes.insert("propagate", "propagates");
    prefix_substitutes.insert("populate", "populates");
    prefix_substitutes.insert("receive", "receives");
    prefix_substitutes.insert("reset", "resets");
    prefix_substitutes.insert("require", "requires");
    // Ex.: `get_reserve_indicator` -> `must_reserve_indicator`.
    prefix_substitutes.insert("reserve", "must_reserve");
    prefix_substitutes.insert("resize", "resizes");
    prefix_substitutes.insert("restrict", "restricts");
    prefix_substitutes.insert("reveal", "reveals");
    prefix_substitutes.insert("select", "selects");
    prefix_substitutes.insert("show", "shows");
    prefix_substitutes.insert("skip", "skips");
    prefix_substitutes.insert("snap", "snaps");
    prefix_substitutes.insert("support", "supports");
    prefix_substitutes.insert("take", "takes");
    prefix_substitutes.insert("track", "tracks");
    // Ex.: `get_truncate_multiline` -> `must_truncate_multiline`.
    prefix_substitutes.insert("truncate", "must_truncate");
    prefix_substitutes.insert("use", "uses");
    prefix_substitutes.insert("wrap", "wraps");
    prefix_substitutes
});

/// Set of `bool` getter suffix prefixes which must not be prefixed with `is`.
///
/// The convention is to rename `bool` getters `get_suffix` as `is_suffix`,
/// but there are cases for which the meaning makes it useless to add the `is` prefix:
///
/// - `get_has_entry` -> `has_entry`.
pub static BOOL_STARTS_WITH_NO_PREFIX: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut bool_as_regular = HashSet::new();
    bool_as_regular.insert("can");
    bool_as_regular.insert("has");
    bool_as_regular.insert("must");
    bool_as_regular.insert("should");
    bool_as_regular.insert("state");
    // Also add all the prefix substitutes (e.g. accepts, skips, ...)
    for bool_substitute in BOOL_PREFIX_SUBSTITUTES.values() {
        bool_as_regular.insert(bool_substitute);
    }
    bool_as_regular
});

/// Special suffix to detect getters returning a `bool`.
///
/// Ex.: `get_seekable`.
pub const BOOL_ABLE_PREFIX: &str = "able";

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
    } else if RESERVED.contains(suffix) {
        Err(RenameError::Reserved)
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
    if let Some(substitute) = BOOL_EXACT_SUBSTITUTES.get(suffix) {
        return NewName::Substituted(substitute.to_string());
    }

    if let Some(new_name) = try_rename_bool_getter(suffix) {
        new_name
    } else {
        NewName::Regular(format!("is_{}", suffix))
    }
}

/// Attempts to apply special rules to the `bool` getter.
///
/// The substitutions are defined in [`BOOL_PREFIX_SUBSTITUTES`]
/// and [`BOOL_STARTS_WITH_NO_PREFIX`].
#[inline]
fn try_rename_bool_getter(suffix: &str) -> Option<NewName> {
    let mut working_suffix = suffix;
    let mut has_is_prefix = false;

    if let Some(suffix_without_is) = suffix.strip_prefix("is_") {
        working_suffix = suffix_without_is;
        has_is_prefix = true;
    }

    let splits: Vec<&str> = working_suffix.splitn(2, '_').collect();
    BOOL_PREFIX_SUBSTITUTES
        .get(splits[0])
        .map(|substitute| {
            if splits.len() == 1 {
                NewName::Substituted(substitute.to_string())
            } else {
                NewName::Substituted(format!("{}_{}", substitute, splits[1]))
            }
        })
        .or_else(|| {
            BOOL_STARTS_WITH_NO_PREFIX.get(splits[0]).map(|_| {
                if splits.len() == 1 {
                    NewName::NoPrefix(splits[0].to_string())
                } else {
                    NewName::NoPrefix(format!("{}_{}", splits[0], splits[1]))
                }
            })
        })
        .or_else(|| {
            // No bool rules applied to the working suffix
            if has_is_prefix {
                // but the suffix was already `is` prefixed
                Some(NewName::Regular(suffix.to_string()))
            } else {
                None
            }
        })
}

/// Attempts to determine whether the getter returns a `bool` from its name.
///
/// Uses [`BOOL_PREFIX_SUBSTITUTES`], [`BOOL_STARTS_WITH_NO_PREFIX`] and
/// [`BOOL_ABLE_PREFIX`] as a best effort estimation.
///
/// Returns the name substitute if `self` seems to be returning a `bool`.
#[inline]
pub fn guesstimate_boolness_then_rename(suffix: &str) -> Option<NewName> {
    if let Some(new_name) = try_rename_bool_getter(suffix) {
        return Some(new_name);
    }

    let splits: Vec<&str> = suffix.splitn(2, '_').collect();
    if splits[0].ends_with(BOOL_ABLE_PREFIX) {
        Some(NewName::Regular(format!("is_{}", suffix)))
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
    /// Fixed name to comply to rules. Ex. `get_mut_structure` -> `structure_mut`.
    Fixed(String),
    /// Regular rule: removal of the prefix or replacement with `is`.
    Regular(String),
    /// No prefix for `bool` getter. Ex. `get_has_entry` -> `has_entry`.
    NoPrefix(String),
    /// Applied substitution. Ex. `get_mute` -> `is_muted`.
    Substituted(String),
}

impl NewName {
    /// Returns the new name.
    pub fn as_str(&self) -> &str {
        use NewName::*;
        match self {
            Fixed(new_name) | Substituted(new_name) | Regular(new_name) | NoPrefix(new_name) => {
                new_name.as_str()
            }
        }
    }

    /// Consumes the [`NewName`] and returns the inner new name [`String`].
    pub fn into_string(self) -> String {
        use NewName::*;
        match self {
            Fixed(new_name) | Substituted(new_name) | Regular(new_name) | NoPrefix(new_name) => {
                new_name
            }
        }
    }

    /// Returns whether renaming required fixing the name to comply with rules.
    ///
    /// Ex. `get_mut_structure` -> `structure_mut`.
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
        matches!(self, NewName::Regular(_))
    }

    /// Returns whether renaming didn't use the `is` prefix for `bool` getter.
    ///
    /// Ex.:
    // * `get_has_entry` -> `has_entry`.
    pub fn is_no_prefix(&self) -> bool {
        matches!(self, NewName::NoPrefix(_))
    }
}

impl Display for NewName {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use NewName::*;
        match self {
            Fixed(new_name) => write!(f, "fixed as {}", new_name),
            Substituted(new_name) => write!(f, "substituted with {}", new_name),
            NoPrefix(new_name) => write!(f, "kept as {}", new_name),
            Regular(new_name) => write!(f, "renamed as {}", new_name),
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
    fn bool_getter_rename_attempt() {
        let new_name = try_rename_bool_getter(&"mute").unwrap();
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "is_muted");

        let new_name = try_rename_bool_getter(&"emit_eos").unwrap();
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_bool_getter(&"has_entry").unwrap();
        assert!(new_name.is_no_prefix());
        assert_eq!(new_name, "has_entry");

        let new_name = try_rename_bool_getter(&"is_emit_eos").unwrap();
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_bool_getter(&"is_activated").unwrap();
        assert!(new_name.is_regular());
        assert_eq!(new_name, "is_activated");

        assert!(try_rename_bool_getter(&"name").is_none());
    }

    #[test]
    fn bool_getter_suffix() {
        let new_name = rename_bool_getter(&"result");
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "result");

        let new_name = rename_bool_getter(&"activable");
        assert!(new_name.is_regular());
        assert_eq!(new_name, "is_activable");

        let new_name = rename_bool_getter(&"mute");
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "is_muted");

        let new_name = rename_bool_getter(&"emit_eos");
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "emits_eos");

        let new_name = rename_bool_getter(&"can_acquire");
        assert!(new_name.is_no_prefix());
        assert_eq!(new_name, "can_acquire");
    }

    #[test]
    fn boolness_guestimation() {
        assert!(guesstimate_boolness_then_rename(&"result").is_none());
        assert!(guesstimate_boolness_then_rename(&"name").is_none());

        let new_name = guesstimate_boolness_then_rename(&"mute").unwrap();
        assert_eq!(new_name, "is_muted");

        let new_name = guesstimate_boolness_then_rename(&"does_ts").unwrap();
        assert_eq!(new_name, "does_ts");

        let new_name = guesstimate_boolness_then_rename(&"emit_eos").unwrap();
        assert_eq!(new_name, "emits_eos");

        let new_name = guesstimate_boolness_then_rename(&"emits_eos").unwrap();
        assert_eq!(new_name, "emits_eos");

        let new_name = guesstimate_boolness_then_rename(&"is_emits_eos").unwrap();
        assert_eq!(new_name, "emits_eos");

        let new_name = guesstimate_boolness_then_rename(&"is_activated").unwrap();
        assert_eq!(new_name, "is_activated");

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
        assert!(new_name.is_regular());
        assert_eq!(new_name, "is_structure");

        let new_name = try_rename_would_be_getter(&"get_mute", true).unwrap();
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "is_muted");

        let new_name = try_rename_would_be_getter(&"get_emit_eos", true).unwrap();
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_would_be_getter(&"get_emits_eos", true).unwrap();
        assert!(new_name.is_no_prefix());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_would_be_getter(&"get_is_emit_eos", true).unwrap();
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_would_be_getter(&"get_is_activated", true).unwrap();
        assert!(new_name.is_regular());
        assert_eq!(new_name, "is_activated");

        let new_name = try_rename_would_be_getter(&"get_activable", true).unwrap();
        assert!(new_name.is_regular());
        assert_eq!(new_name, "is_activable");

        let new_name = try_rename_would_be_getter(&"get_mut", true).unwrap();
        assert!(new_name.is_regular());
        assert_eq!(new_name, "is_mut");

        let new_name = try_rename_would_be_getter(&"get_overwrite", true).unwrap();
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "overwrites");

        let new_name = try_rename_would_be_getter(&"get_overwrite_mode", true).unwrap();
        assert!(new_name.is_regular());
        assert_eq!(new_name, "is_overwrite_mode");

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

        let new_name = try_rename_would_be_getter(&"get_emits_eos", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_no_prefix());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_would_be_getter(&"get_is_emit_eos", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_substituted());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_would_be_getter(&"get_is_activated", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_regular());
        assert_eq!(new_name, "is_activated");

        let new_name = try_rename_would_be_getter(&"get_activable", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_regular());
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
