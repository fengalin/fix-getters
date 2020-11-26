use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::{
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
        reserved.insert("loop");
        reserved.insert("mut");
        reserved.insert("optional");
        reserved.insert("owned");
        reserved.insert("some");
        reserved.insert("type");
        reserved
    };

    /// Special names for getters returning a boolean.
    ///
    /// The convention is to rename `get_active` as `is_active`,
    /// but there are cases for which we want a better name:
    ///
    /// - `get_mute` -> `is_muted`.
    /// - `get_emit_eos` -> `emits_eos`.
    ///
    /// This is intended at renaming to fit the convention.
    pub static ref BOOL_PREFIX_MAP: HashMap<&'static str, &'static str> ={
        let mut bool_prefix_map = HashMap::new();
        bool_prefix_map.insert("do", "does");
        bool_prefix_map.insert("emit", "emits");
        bool_prefix_map.insert("fill", "fills");
        bool_prefix_map.insert("mute", "is_muted");
        bool_prefix_map.insert("reset", "resets");
        bool_prefix_map.insert("result", "result");
        bool_prefix_map
    };

    /// Getters prefix to move to the end.
    ///
    /// The convention is to use the form:
    /// - `get_structure_mut`.
    ///
    /// but we can run into this one too:
    /// - `get_mut_structure`.
    ///
    /// This is intended at renaming to fit the convention.
    pub static ref PREFIX_TO_POSTFIX: HashSet<&'static str> ={
        let mut prefix_to_postfix = HashSet::new();
        prefix_to_postfix.insert("mut");
        prefix_to_postfix
    };
}

/// Applies getter name rules to the given getter suffix.
///
/// Suffix in the form of `get_suffix`.
///
/// The `is_bool_getter` function will be executed if it is
/// necessary to check whether the getter returns exactly a bool.
#[inline]
pub fn try_rename_getter_suffix<F>(suffix: &str, is_bool_getter: F) -> Result<RenameOk, RenameError>
where
    F: FnOnce() -> bool,
{
    if RESERVED.contains(suffix) {
        return Err(RenameError::Reserved);
    }

    if is_bool_getter() {
        return Ok(rename_bool_getter(suffix));
    }

    let splits: Vec<&str> = suffix.splitn(2, '_').collect();
    if splits.len() > 1 && PREFIX_TO_POSTFIX.contains(splits[0]) {
        Ok(RenameOk::Fixed(format!("{}_{}", splits[1], splits[0])))
    } else {
        Ok(RenameOk::Unchanged(suffix.to_string()))
    }
}

/// Applies boolean getter name rules.
///
/// Suffix in the form of `get_suffix`.
#[inline]
pub fn rename_bool_getter(suffix: &str) -> RenameOk {
    try_substitute(suffix)
        .map(RenameOk::Subsituted)
        .unwrap_or_else(|| RenameOk::Fixed(format!("is_{}", suffix)))
}

/// Attempts to apply substitutions to the given boolean getter suffix.
///
/// Suffix in the form of `get_suffix`.
#[inline]
pub fn try_substitute(suffix: &str) -> Option<String> {
    let splits: Vec<&str> = suffix.splitn(2, '_').collect();
    BOOL_PREFIX_MAP.get(splits[0]).map(|substitute| {
        if splits.len() == 1 {
            substitute.to_string()
        } else {
            format!("{}_{}", substitute, splits[1])
        }
    })
}

/// Checks the rules against the given function signature.
pub fn try_rename_getter(sig: &syn::Signature) -> Result<RenameOk, RenameError> {
    use RenameError::*;

    let name = sig.ident.to_string();
    let suffix = name.strip_prefix("get_");

    let suffix = match suffix {
        Some(suffix) => suffix,
        None => return Err(NotAGet),
    };

    let syn::Generics { params, .. } = &sig.generics;
    if !params.is_empty() {
        return Err(GenericParams);
    }

    if sig.inputs.len() > 1 {
        return Err(MultipleArgs);
    }

    match sig.inputs.first() {
        Some(syn::FnArg::Receiver { .. }) => (),
        Some(_) => return Err(OneNoneSelfArg),
        None => return Err(NoArgs),
    }

    try_rename_getter_suffix(suffix, || returns_bool(sig))
}

#[inline]
fn returns_bool(sig: &syn::Signature) -> bool {
    if let syn::ReturnType::Type(_, type_) = &sig.output {
        if let syn::Type::Path(syn::TypePath { path, .. }) = type_.as_ref() {
            if path.segments.len() == 1 {
                if let Some(syn::PathSegment { ident, .. }) = &path.segments.first() {
                    if ident == "bool" {
                        return true;
                    }
                }
            }
        }
    }

    false
}

/// Suffix renaming successfull Result.
#[derive(Debug)]
#[non_exhaustive]
pub enum RenameOk {
    /// Suffix was fixed to comply to rules. Ex. `get_active` -> `is_active`.
    Fixed(String),
    /// Suffix was fixed with substitution. Ex. `get_mute` -> `is_muted`.
    Subsituted(String),
    /// Suffix is unchanged.
    Unchanged(String),
}

impl RenameOk {
    pub fn into_inner(self) -> String {
        match self {
            RenameOk::Fixed(inner) | RenameOk::Subsituted(inner) | RenameOk::Unchanged(inner) => {
                inner
            }
        }
    }

    pub fn inner(&self) -> &str {
        match self {
            RenameOk::Fixed(inner) | RenameOk::Subsituted(inner) | RenameOk::Unchanged(inner) => {
                inner.as_str()
            }
        }
    }

    pub fn is_fixed(&self) -> bool {
        matches!(self, RenameOk::Fixed(_))
    }

    pub fn is_substituted(&self) -> bool {
        matches!(self, RenameOk::Subsituted(_))
    }
}

/// Suffix renaming failure Result.
#[derive(Debug)]
#[non_exhaustive]
pub enum RenameError {
    GenericParams,
    MultipleArgs,
    NotAGet,
    NoArgs,
    OneNoneSelfArg,
    Reserved,
}

impl Display for RenameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use RenameError::*;

        match self {
            GenericParams => f.write_str("generic parameters"),
            MultipleArgs => f.write_str("multiple arguments"),
            NotAGet => f.write_str("not a get function"),
            NoArgs => f.write_str("no arguments"),
            OneNoneSelfArg => f.write_str("none `self` one argument"),
            Reserved => f.write_str("name is reserved"),
        }
    }
}

impl Error for RenameError {}
