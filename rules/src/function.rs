//! Would-be-getter renaming rules definition.

use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::{self, Display},
};

lazy_static! {
    /// Getters reserved suffix list.
    ///
    /// Getter that we don't want to rename because
    /// they are reserved words or would result confusing.
    pub static ref RESERVED: HashSet<&'static str> ={
        let mut reserved = HashSet::new();
        reserved.insert("");
        reserved.insert("impl");
        reserved.insert("loop");
        reserved.insert("mut");
        reserved.insert("optional");
        reserved.insert("owned");
        reserved.insert("ref");
        reserved.insert("some");
        reserved.insert("type");
        reserved
    };

    /// Substitutes for getters returning a `bool`.
    ///
    /// The convention is to rename `bool` getters `get_suffix` as `is_suffix`,
    /// but there are cases for which we want a better name:
    ///
    /// - `get_mute` -> `is_muted`.
    /// - `get_emit_eos` -> `emits_eos`.
    pub static ref BOOL_SUBSTITUTES: HashMap<&'static str, &'static str> ={
        let mut bool_substitutes = HashMap::new();
        bool_substitutes.insert("do", "does");
        bool_substitutes.insert("emit", "emits");
        bool_substitutes.insert("fill", "fills");
        bool_substitutes.insert("mute", "is_muted");
        bool_substitutes.insert("reset", "resets");
        bool_substitutes.insert("show", "shows");
        bool_substitutes
    };

    /// Set of getters returning a `bool` which should be handle as regular getters.
    ///
    /// The convention is to rename `bool` getters `get_suffix` as `is_suffix`,
    /// but there are cases for which we want to apply the regular getter rule:
    ///
    /// - `get_result` -> `result`.
    pub static ref BOOL_AS_REGULAR: HashSet<&'static str> ={
        let mut bool_as_regular = HashSet::new();
        bool_as_regular.insert("result");
        bool_as_regular
    };

    /// Getters prefix to move to the end.
    ///
    /// The convention is to use the form:
    /// - `get_structure_mut`.
    ///
    /// but we can run into this one too:
    /// - `get_mut_structure`.
    pub static ref PREFIX_TO_POSTFIX: HashSet<&'static str> ={
        let mut prefix_to_postfix = HashSet::new();
        prefix_to_postfix.insert("mut");
        prefix_to_postfix
    };
}

/// Special suffix to detect getters returning a `bool`.
///
/// Ex.: `get_seekable`.
pub const BOOL_ABLE_PREFIX: &str = "able";

/// Attempts to apply getter name rules to this function name.
///
/// The argument `returns_bool` hints the renaming process when
/// the getter returns a unique `bool` value. Use [`ReturnsBool::Maybe`]
/// if the return value is not known.
pub fn try_rename_getter(
    name: &str,
    returns_bool: impl Into<ReturnsBool>,
) -> Result<NewName, RenameError> {
    let suffix = match name.strip_prefix("get_") {
        Some(suffix) => suffix,
        None => return Err(RenameError::NotGetFn),
    };

    if RESERVED.contains(suffix) {
        return Err(RenameError::Reserved);
    }

    use ReturnsBool::*;
    match returns_bool.into() {
        False => (),
        True => return Ok(rename_bool_getter(suffix)),
        Maybe => {
            if let Some(rename) = guesstimate_boolness(suffix) {
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

/// Applies `bool` getter name rules.
#[inline]
pub fn rename_bool_getter(suffix: &str) -> NewName {
    if BOOL_AS_REGULAR.contains(suffix) {
        NewName::Regular(suffix.to_string())
    } else if let Some(new_name) = try_substitute(suffix) {
        NewName::Substituted(new_name)
    } else {
        NewName::Fixed(format!("is_{}", suffix))
    }
}

/// Attempts to apply special substitutions for boolean getters.
///
/// The substitutions are defined in [`BOOL_SUBSTITUTES`](`struct@BOOL_SUBSTITUTES`).
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
/// Uses the boolean prefix map and [`BOOL_ABLE_PREFIX`] as a best effort estimation.
///
/// Returns the name substitute if `self` seems to be returning a `bool`.
#[inline]
pub fn guesstimate_boolness(suffix: &str) -> Option<NewName> {
    if let Some(new_name) = try_substitute(suffix) {
        return Some(NewName::Substituted(new_name));
    }

    let splits: Vec<&str> = suffix.splitn(2, '_').collect();
    if splits[0].ends_with(BOOL_ABLE_PREFIX) {
        Some(NewName::Substituted(format!("is_{}", suffix)))
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
    /// Regaular removal of the prefix.
    Regular(String),
}

impl NewName {
    /// Returns the new name.
    pub fn as_str(&self) -> &str {
        use NewName::*;
        match self {
            Fixed(new_name) | Substituted(new_name) | Regular(new_name) => new_name.as_str(),
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

    /// Returns whether renaming used the regular prefix removal strategy.
    ///
    /// Ex. `get_name` -> `name`.
    pub fn is_regular(&self) -> bool {
        matches!(self, NewName::Regular(_))
    }

    /// Returns the inner the new name as [`String`].
    pub fn into_inner(self) -> String {
        use NewName::*;
        match self {
            Fixed(new_name) | Substituted(new_name) | Regular(new_name) => new_name,
        }
    }
}

impl Display for NewName {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use NewName::*;
        match self {
            Fixed(new_name) => write!(f, "fixed as {}", new_name),
            Substituted(new_name) => write!(f, "substituted with {}", new_name),
            Regular(new_name) => write!(f, "renamed as {}", new_name),
        }
    }
}

impl PartialEq<str> for NewName {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
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
