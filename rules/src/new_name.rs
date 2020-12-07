//! Would-be-getter rename attempt sucessful result and details.

use std::fmt::{self, Display};

/// Would-be-getter rename attempt sucessful result and details.
///
/// Holds details about what happened and assumptions on the return type.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub struct NewName {
    pub(crate) new_name: String,
    pub(crate) returns_bool: ReturnsBool,
    pub(crate) rule: NewNameRule,
}

impl NewName {
    /// Returns the new name.
    pub fn as_str(&self) -> &str {
        self.new_name.as_str()
    }

    /// Consumes the [`NewName`] and returns the inner new name [`String`].
    pub fn unwrap(self) -> String {
        self.new_name
    }

    /// Returns current knowledge about the getter returning exactly one `bool`.
    pub fn returns_bool(&self) -> ReturnsBool {
        self.returns_bool
    }

    /// Returns the renaming rule that was used to rename the getter.
    pub fn rule(&self) -> NewNameRule {
        self.rule
    }

    /// Returns whether renaming required fixing the name to comply with rules.
    ///
    /// Ex. `get_mut_structure` -> `structure_mut`.
    pub fn is_fixed(&self) -> bool {
        self.rule.is_fixed()
    }

    /// Returns whether renaming required substituing (part) of the name.
    ///
    /// Ex. `get_mute` -> `is_muted`.
    pub fn is_substituted(&self) -> bool {
        self.rule.is_substituted()
    }

    /// Returns whether renaming used the regular strategy.
    ///
    /// Ex.:
    // * `get_name` -> `name`.
    // * `get_active` -> `is_active`.
    pub fn is_regular(&self) -> bool {
        self.rule.is_regular()
    }

    /// Returns whether renaming didn't use the `is` prefix for `bool` getter.
    ///
    /// Ex.:
    // * `get_has_entry` -> `has_entry`.
    pub fn is_no_prefix(&self) -> bool {
        self.rule.is_no_prefix()
    }
}

impl Display for NewName {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}{} {}", self.returns_bool, self.rule, self.new_name)
    }
}

impl<T: AsRef<str>> PartialEq<T> for NewName {
    fn eq(&self, other: &T) -> bool {
        self.as_str() == other.as_ref()
    }
}

/// Rule that applied to get the [`NewName`].
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum NewNameRule {
    /// Fixed name to comply to rules. Ex. `get_mut_structure` -> `structure_mut`.
    Fixed,
    /// Regular rule: removal of the prefix or replacement with `is`.
    Regular,
    /// No prefix for `bool` getter. Ex. `get_has_entry` -> `has_entry`.
    NoPrefix,
    /// Applied substitution. Ex. `get_mute` -> `is_muted`.
    Substituted,
}

impl NewNameRule {
    /// Returns whether renaming required fixing the name to comply with rules.
    ///
    /// Ex. `get_mut_structure` -> `structure_mut`.
    pub fn is_fixed(&self) -> bool {
        matches!(self, NewNameRule::Fixed)
    }

    /// Returns whether renaming required substituing (part) of the name.
    ///
    /// Ex. `get_mute` -> `is_muted`.
    pub fn is_substituted(&self) -> bool {
        matches!(self, NewNameRule::Substituted)
    }

    /// Returns whether renaming used the regular strategy.
    ///
    /// Ex.:
    // * `get_name` -> `name`.
    // * `get_active` -> `is_active`.
    pub fn is_regular(&self) -> bool {
        matches!(self, NewNameRule::Regular)
    }

    /// Returns whether renaming didn't use the `is` prefix for `bool` getter.
    ///
    /// Ex.:
    // * `get_has_entry` -> `has_entry`.
    pub fn is_no_prefix(&self) -> bool {
        matches!(self, NewNameRule::NoPrefix)
    }
}

impl Display for NewNameRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use NewNameRule::*;
        match self {
            Fixed => f.write_str("fixed as"),
            Substituted => f.write_str("substituted with"),
            NoPrefix => f.write_str("kept as"),
            Regular => f.write_str("renamed as"),
        }
    }
}

/// Indicates current knowledge of the get function returning exaclty one `bool`.
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

    pub fn is_false(&self) -> bool {
        matches!(self, ReturnsBool::False)
    }

    pub fn is_maybe(&self) -> bool {
        matches!(self, ReturnsBool::Maybe)
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

impl Display for ReturnsBool {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use ReturnsBool::*;
        match self {
            False => Ok(()),
            True => f.write_str("-> bool "),
            Maybe => f.write_str("-> ? "),
        }
    }
}
