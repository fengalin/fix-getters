//! Would-be-getter renaming rules definition.

use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::{self, Display},
};

use crate::{NewName, NewNameRule, ReturnsBool};

/// Getters reserved suffix list.
///
/// Getter that we don't want to rename because
/// they are Rust keywords or would result confusing.
pub static RESERVED: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut reserved = HashSet::new();
    reserved.insert("");
    reserved.insert("as");
    reserved.insert("const"); // keep `get_const` similar to `get`
    reserved.insert("else");
    reserved.insert("false");
    reserved.insert("for");
    reserved.insert("if");
    reserved.insert("in");
    reserved.insert("mut"); // keep `get_mut` similar to `get`
    reserved.insert("optional"); // keep `get_optional` similar to `get`
    reserved.insert("owned"); // keep `get_owned` similar to `get`
    reserved.insert("ref"); // keep `get_ref` similar to `get`
    reserved.insert("some"); // keep `get_some` similar to `get`
    reserved.insert("true");
    reserved.insert("unchecked_mut"); // don't change call-sites: used in various Rust types
    reserved.insert("where");
    reserved.insert("while");
    reserved
});

/// Substitutes to be used when the suffix matches exactly.
///
/// The convention is to rename getters `get_suffix` as `suffix`,
/// but there are cases for which a better name should be used:
///
/// - `get_type` -> `type_`.
pub static EXACT_SUFFIX_SUBSTITUTES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut exact_subs = HashMap::new();
    exact_subs.insert("async", "async_");
    exact_subs.insert("await", "await_");
    exact_subs.insert("break", "break_");
    exact_subs.insert("crate", "crate_");
    exact_subs.insert("continue", "continue_");
    exact_subs.insert("dyn", "dyn_");
    exact_subs.insert("enum", "enum_");
    exact_subs.insert("extern", "extern_");
    exact_subs.insert("fn", "fn_");
    exact_subs.insert("impl", "impl_");
    exact_subs.insert("loop", "loop_");
    exact_subs.insert("match", "match_");
    exact_subs.insert("mod", "mod_");
    exact_subs.insert("move", "move_");
    exact_subs.insert("pub", "pub_");
    exact_subs.insert("return", "return_");
    exact_subs.insert("self", "self_");
    exact_subs.insert("static", "static_");
    exact_subs.insert("struct", "struct_");
    exact_subs.insert("super", "super_");
    exact_subs.insert("trait", "trait_");
    exact_subs.insert("type", "type_");
    exact_subs.insert("union", "union_");
    exact_subs.insert("unsafe", "unsafe_");
    exact_subs.insert("use", "use_");
    exact_subs
});

/// Substitutes for tokens of `bool` getters.
///
/// The convention is to rename `bool` getters `get_suffix` as `is_suffix`,
/// but there are cases for which we want a better name:
///
/// - `get_mute` -> `is_muted`.
/// - `get_emit_eos` -> `emits_eos`.
pub static BOOL_FIRST_TOKEN_SUBSTITUTES: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut first_token_subs = HashMap::new();
        first_token_subs.insert("activate", "activates");
        first_token_subs.insert("accept", "accepts");
        first_token_subs.insert("allow", "allows");
        // Ex.: `get_always_show_image` -> `must_always_show_image`.
        first_token_subs.insert("always", "must_always");
        first_token_subs.insert("close", "closes");
        first_token_subs.insert("create", "creates");
        // Ex.: `get_destroy_with_parent` -> `must_destroy_with_parent`.
        first_token_subs.insert("destroy", "must_destroy");
        first_token_subs.insert("do", "does");
        first_token_subs.insert("draw", "draws");
        first_token_subs.insert("embed", "embeds");
        first_token_subs.insert("emit", "emits");
        first_token_subs.insert("enable", "enables");
        first_token_subs.insert("exit", "exits");
        first_token_subs.insert("expand", "expands");
        first_token_subs.insert("fill", "fills");
        first_token_subs.insert("fit", "fits");
        // Ex.: `get_focus_on_map` -> `gets_focus_on_map`.
        first_token_subs.insert("focus", "gets_focus");
        first_token_subs.insert("follow", "follows");
        first_token_subs.insert("hide", "hides");
        first_token_subs.insert("ignore", "ignores");
        first_token_subs.insert("invert", "inverts");
        first_token_subs.insert("mute", "is_muted");
        first_token_subs.insert("need", "needs");
        first_token_subs.insert("propagate", "propagates");
        first_token_subs.insert("populate", "populates");
        first_token_subs.insert("receive", "receives");
        first_token_subs.insert("reset", "resets");
        first_token_subs.insert("require", "requires");
        // Ex.: `get_reserve_indicator` -> `must_reserve_indicator`.
        first_token_subs.insert("reserve", "must_reserve");
        first_token_subs.insert("resize", "resizes");
        first_token_subs.insert("restrict", "restricts");
        first_token_subs.insert("reveal", "reveals");
        first_token_subs.insert("select", "selects");
        first_token_subs.insert("show", "shows");
        first_token_subs.insert("shrink", "shrinks");
        first_token_subs.insert("skip", "skips");
        first_token_subs.insert("snap", "snaps");
        first_token_subs.insert("support", "supports");
        first_token_subs.insert("take", "takes");
        first_token_subs.insert("track", "tracks");
        // Ex.: `get_truncate_multiline` -> `must_truncate_multiline`.
        first_token_subs.insert("truncate", "must_truncate");
        first_token_subs.insert("use", "uses");
        first_token_subs.insert("wrap", "wraps");
        first_token_subs
    });

/// Set of `bool` getter suffix first tokens for which no be prefix should be applied.
///
/// The convention is to rename `bool` getters `get_suffix` as `is_suffix`,
/// but there are cases for which the meaning makes it useless to add the `is` prefix:
///
/// - `get_has_entry` -> `has_entry`.
pub static BOOL_FIRST_TOKEN_NO_PREFIX: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut first_tokens = HashSet::new();
    first_tokens.insert("can");
    first_tokens.insert("has");
    first_tokens.insert("must");
    first_tokens.insert("should");
    first_tokens.insert("state");
    // Also add all the prefix substitutes (e.g. accepts, skips, ...)
    for bool_substitute in BOOL_FIRST_TOKEN_SUBSTITUTES.values() {
        first_tokens.insert(bool_substitute);
    }
    first_tokens
});

/// Substitutes of `bool` getter to be used when the suffix matches exactly.
///
/// The convention is to rename `bool` getters `get_suffix` as `is_suffix`,
/// but there are cases for which it would be confusing to add the `is` prefix:
///
/// - `get_result` -> `result`.
/// - `get_overwrite` -> `overwrites`. Note that if the getter suffix doesn't match
///   exactly, this rule doesn't apply. Ex. `get_overwrite_mode` -> `is_overwrites_mode`
pub static BOOL_EXACT_SUBSTITUTES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut exact_subs = HashMap::new();
    exact_subs.insert("result", "result");
    exact_subs.insert("overwrite", "overwrites");
    exact_subs.insert("visibility", "is_visible");
    exact_subs
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
    let suffix = match name.strip_prefix("get_") {
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
    let returns_bool = match returns_bool.into() {
        False => ReturnsBool::False,
        True => return Ok(rename_bool_getter(suffix)),
        Maybe => {
            if let Some(rename) = guesstimate_boolness_then_rename(suffix) {
                return Ok(rename);
            }
            ReturnsBool::Maybe
        }
    };

    if let Some(substitute) = EXACT_SUFFIX_SUBSTITUTES.get(suffix) {
        return Ok(NewName {
            new_name: substitute.to_string(),
            returns_bool,
            rule: NewNameRule::Substituted,
        });
    }

    let splits: Vec<&str> = suffix.splitn(2, '_').collect();
    if splits.len() > 1 && PREFIX_TO_POSTFIX.contains(splits[0]) {
        Ok(NewName {
            new_name: format!("{}_{}", splits[1], splits[0]),
            returns_bool,
            rule: NewNameRule::Fixed,
        })
    } else if RESERVED.contains(suffix) {
        Err(RenameError::Reserved)
    } else {
        Ok(NewName {
            new_name: suffix.to_string(),
            returns_bool,
            rule: NewNameRule::Regular,
        })
    }
}

/// Applies `bool` getter name rules.
#[inline]
pub fn rename_bool_getter(suffix: &str) -> NewName {
    if let Some(substitute) = BOOL_EXACT_SUBSTITUTES.get(suffix) {
        return NewName {
            new_name: substitute.to_string(),
            returns_bool: true.into(),
            rule: NewNameRule::Substituted,
        };
    }

    if let Some(new_name) = try_rename_bool_getter(suffix) {
        new_name
    } else {
        NewName {
            new_name: format!("is_{}", suffix),
            returns_bool: true.into(),
            rule: NewNameRule::Regular,
        }
    }
}

/// Attempts to apply special rules to the `bool` getter.
///
/// The substitutions are defined in [`BOOL_FIRST_TOKEN_SUBSTITUTES`]
/// and [`BOOL_FIRST_TOKEN_NO_PREFIX`].
#[inline]
fn try_rename_bool_getter(suffix: &str) -> Option<NewName> {
    let mut working_suffix = suffix;
    let mut has_is_prefix = false;

    if let Some(suffix_without_is) = suffix.strip_prefix("is_") {
        working_suffix = suffix_without_is;
        has_is_prefix = true;
    }

    let splits: Vec<&str> = working_suffix.splitn(2, '_').collect();
    BOOL_FIRST_TOKEN_SUBSTITUTES
        .get(splits[0])
        .map(|substitute| {
            if splits.len() == 1 {
                NewName {
                    new_name: substitute.to_string(),
                    returns_bool: true.into(),
                    rule: NewNameRule::Substituted,
                }
            } else {
                NewName {
                    new_name: format!("{}_{}", substitute, splits[1]),
                    returns_bool: true.into(),
                    rule: NewNameRule::Substituted,
                }
            }
        })
        .or_else(|| {
            BOOL_FIRST_TOKEN_NO_PREFIX.get(splits[0]).map(|_| {
                if splits.len() == 1 {
                    NewName {
                        new_name: splits[0].to_string(),
                        returns_bool: true.into(),
                        rule: NewNameRule::NoPrefix,
                    }
                } else {
                    NewName {
                        new_name: format!("{}_{}", splits[0], splits[1]),
                        returns_bool: true.into(),
                        rule: NewNameRule::NoPrefix,
                    }
                }
            })
        })
        .or_else(|| {
            // No bool rules applied to the working suffix
            if has_is_prefix {
                // but the suffix was already `is` prefixed
                Some(NewName {
                    new_name: suffix.to_string(),
                    returns_bool: true.into(),
                    rule: NewNameRule::Regular,
                })
            } else {
                None
            }
        })
}

/// Attempts to determine whether the getter returns a `bool` from its name.
///
/// Uses [`BOOL_FIRST_TOKEN_SUBSTITUTES`], [`BOOL_FIRST_TOKEN_NO_PREFIX`] and
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
        Some(NewName {
            new_name: format!("is_{}", suffix),
            returns_bool: true.into(),
            rule: NewNameRule::Regular,
        })
    } else {
        None
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
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_muted");

        let new_name = try_rename_bool_getter(&"emit_eos").unwrap();
        assert!(new_name.is_substituted());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_bool_getter(&"has_entry").unwrap();
        assert!(new_name.is_no_prefix());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "has_entry");

        let new_name = try_rename_bool_getter(&"is_emit_eos").unwrap();
        assert!(new_name.is_substituted());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_bool_getter(&"is_activated").unwrap();
        assert!(new_name.is_regular());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_activated");

        assert!(try_rename_bool_getter(&"name").is_none());
    }

    #[test]
    fn bool_getter_suffix() {
        let new_name = rename_bool_getter(&"result");
        assert!(new_name.is_substituted());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "result");

        let new_name = rename_bool_getter(&"activable");
        assert!(new_name.is_regular());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_activable");

        let new_name = rename_bool_getter(&"mute");
        assert!(new_name.is_substituted());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_muted");

        let new_name = rename_bool_getter(&"emit_eos");
        assert!(new_name.is_substituted());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "emits_eos");

        let new_name = rename_bool_getter(&"can_acquire");
        assert!(new_name.is_no_prefix());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "can_acquire");
    }

    #[test]
    fn boolness_guestimation() {
        assert!(guesstimate_boolness_then_rename(&"result").is_none());
        assert!(guesstimate_boolness_then_rename(&"name").is_none());

        let new_name = guesstimate_boolness_then_rename(&"mute").unwrap();
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_muted");

        let new_name = guesstimate_boolness_then_rename(&"does_ts").unwrap();
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "does_ts");

        let new_name = guesstimate_boolness_then_rename(&"emit_eos").unwrap();
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "emits_eos");

        let new_name = guesstimate_boolness_then_rename(&"emits_eos").unwrap();
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "emits_eos");

        let new_name = guesstimate_boolness_then_rename(&"is_emits_eos").unwrap();
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "emits_eos");

        let new_name = guesstimate_boolness_then_rename(&"is_activated").unwrap();
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_activated");

        let new_name = guesstimate_boolness_then_rename(&"activable").unwrap();
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_activable");
    }

    #[test]
    fn rename_getter_non_bool() {
        let new_name = try_rename_would_be_getter(&"get_structure", false).unwrap();
        assert!(new_name.is_regular());
        assert!(new_name.returns_bool().is_false());
        assert_eq!(new_name, "structure");

        let new_name = try_rename_would_be_getter(&"get_type", false).unwrap();
        assert!(new_name.is_substituted());
        assert!(new_name.returns_bool().is_false());
        assert_eq!(new_name, "type_");

        // Bool-alike, but not a bool
        let new_name = try_rename_would_be_getter(&"get_activable", false).unwrap();
        assert!(new_name.is_regular());
        assert!(new_name.returns_bool().is_false());
        assert_eq!(new_name, "activable");

        // Prefix to postfix
        let new_name = try_rename_would_be_getter(&"get_mut_structure", false).unwrap();
        assert!(new_name.is_fixed());
        assert!(new_name.returns_bool().is_false());
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
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_structure");

        let new_name = try_rename_would_be_getter(&"get_type", true).unwrap();
        assert!(new_name.is_regular());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_type");

        let new_name = try_rename_would_be_getter(&"get_mute", true).unwrap();
        assert!(new_name.is_substituted());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_muted");

        let new_name = try_rename_would_be_getter(&"get_emit_eos", true).unwrap();
        assert!(new_name.is_substituted());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_would_be_getter(&"get_emits_eos", true).unwrap();
        assert!(new_name.is_no_prefix());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_would_be_getter(&"get_is_emit_eos", true).unwrap();
        assert!(new_name.is_substituted());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_would_be_getter(&"get_is_activated", true).unwrap();
        assert!(new_name.is_regular());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_activated");

        let new_name = try_rename_would_be_getter(&"get_activable", true).unwrap();
        assert!(new_name.is_regular());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_activable");

        let new_name = try_rename_would_be_getter(&"get_mut", true).unwrap();
        assert!(new_name.is_regular());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_mut");

        let new_name = try_rename_would_be_getter(&"get_overwrite", true).unwrap();
        assert!(new_name.is_substituted());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "overwrites");

        let new_name = try_rename_would_be_getter(&"get_overwrite_mode", true).unwrap();
        assert!(new_name.is_regular());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_overwrite_mode");

        assert!(try_rename_would_be_getter(&"not_a_getter", true)
            .unwrap_err()
            .is_not_get_fn());
    }

    #[test]
    fn rename_getter_maybe_bool() {
        let new_name = try_rename_would_be_getter(&"get_structure", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_regular());
        assert!(new_name.returns_bool().is_maybe());
        assert_eq!(new_name, "structure");

        let new_name = try_rename_would_be_getter(&"get_type", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_substituted());
        assert!(new_name.returns_bool().is_maybe());
        assert_eq!(new_name, "type_");

        let new_name = try_rename_would_be_getter(&"get_mute", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_substituted());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_muted");

        let new_name = try_rename_would_be_getter(&"get_emit_eos", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_substituted());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_would_be_getter(&"get_emits_eos", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_no_prefix());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_would_be_getter(&"get_is_emit_eos", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_substituted());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "emits_eos");

        let new_name = try_rename_would_be_getter(&"get_is_activated", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_regular());
        assert!(new_name.returns_bool().is_true());
        assert_eq!(new_name, "is_activated");

        let new_name = try_rename_would_be_getter(&"get_activable", ReturnsBool::Maybe).unwrap();
        assert!(new_name.is_regular());
        assert!(new_name.returns_bool().is_true());
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
