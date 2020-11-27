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

/// Special suffix to detect getters returning a boolean.
///
/// Ex.: `get_seekable`.
pub const BOOL_ABLE_PREFIX: &str = "able";

/// A function which prefix is `get_`.
#[derive(Debug)]
pub struct GetFunction {
    name: String,
    suffix: String,
}

impl GetFunction {
    /// Attempts to extract the getter suffix from a function name.
    pub fn try_from(name: String) -> Result<Self, GetFunctionError> {
        if let Some(suffix) = name.strip_prefix("get_") {
            Ok(GetFunction {
                suffix: suffix.to_string(),
                name,
            })
        } else {
            Err(GetFunctionError(name))
        }
    }

    /// Attempts to apply getter name rules to this get function.
    ///
    /// The `returns_bool` function will be executed if it is
    /// necessary to check whether the getter returns exactly a bool.
    pub fn try_rename<F>(self, returns_bool: F) -> Result<RenameOk, RenameError>
    where
        F: FnOnce() -> ReturnsBool,
    {
        if RESERVED.contains(self.suffix.as_str()) {
            return Err(RenameError::Reserved(self.name));
        }

        use ReturnsBool::*;
        match returns_bool() {
            False => (),
            True => return Ok(self.rename_bool_getter()),
            Maybe => {
                if let Some(rename_ok) = self.guesstimate_boolness() {
                    return Ok(rename_ok);
                }
            }
        }

        let splits: Vec<&str> = self.suffix.splitn(2, '_').collect();
        if splits.len() > 1 && PREFIX_TO_POSTFIX.contains(splits[0]) {
            Ok(RenameOk::Fix {
                name: self.name,
                new_name: format!("{}_{}", splits[1], splits[0]),
            })
        } else {
            Ok(RenameOk::Regular {
                name: self.name,
                new_name: self.suffix,
            })
        }
    }

    /// Applies boolean getter name rules.
    #[inline]
    pub fn rename_bool_getter(self) -> RenameOk {
        if let Some(new_name) = self.try_substitute() {
            RenameOk::Substitute {
                name: self.name,
                new_name,
            }
        } else {
            RenameOk::Fix {
                name: self.name,
                new_name: format!("is_{}", self.suffix),
            }
        }
    }

    /// Attempts to apply special substitutions for boolean getters.
    ///
    /// The substitutions are defined in [`BOOL_PREFIX_MAP`].
    #[inline]
    pub fn try_substitute(&self) -> Option<String> {
        let splits: Vec<&str> = self.suffix.splitn(2, '_').collect();
        BOOL_PREFIX_MAP.get(splits[0]).map(|substitute| {
            if splits.len() == 1 {
                substitute.to_string()
            } else {
                format!("{}_{}", substitute, splits[1])
            }
        })
    }

    /// Attempts to determine if the getter returns a bool from its name.
    ///
    /// Uses the boolean prefix map and BOOL_ABLE_PREFIX as a best effort estimation.
    ///
    /// Returns the name substitute if it seems to be returning a bool.
    pub fn guesstimate_boolness(&self) -> Option<RenameOk> {
        if let Some(new_name) = self.try_substitute() {
            Some(RenameOk::Substitute {
                name: self.name.clone(),
                new_name,
            })
        } else {
            let splits: Vec<&str> = self.suffix.splitn(2, '_').collect();
            if splits[0].ends_with(BOOL_ABLE_PREFIX) {
                Some(RenameOk::Substitute {
                    name: self.name.clone(),
                    new_name: format!("is_{}", self.suffix()),
                })
            } else {
                None
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn suffix(&self) -> &str {
        &self.suffix
    }

    pub fn into_name(self) -> String {
        self.name
    }
}

pub enum ReturnsBool {
    True,
    False,
    Maybe,
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

/// Checks the rules against the given function signature.
pub fn try_rename_getter_def(sig: &syn::Signature) -> Result<RenameOk, RenameError> {
    let get_fn = GetFunction::try_from(sig.ident.to_string())?;

    let syn::Generics { params, .. } = &sig.generics;
    if !params.is_empty() {
        return Err(RenameError::GenericParams(get_fn.into_name()));
    }

    if sig.inputs.len() > 1 {
        return Err(RenameError::MultipleArgs(get_fn.into_name()));
    }

    match sig.inputs.first() {
        Some(syn::FnArg::Receiver { .. }) => (),
        Some(_) => return Err(RenameError::OneNoneSelfArg(get_fn.into_name())),
        None => return Err(RenameError::NoArgs(get_fn.into_name())),
    }

    get_fn.try_rename(|| returns_bool(sig))
}

#[inline]
fn returns_bool(sig: &syn::Signature) -> ReturnsBool {
    if let syn::ReturnType::Type(_, type_) = &sig.output {
        if let syn::Type::Path(syn::TypePath { path, .. }) = type_.as_ref() {
            if path.segments.len() == 1 {
                if let Some(syn::PathSegment { ident, .. }) = &path.segments.first() {
                    if ident == "bool" {
                        return ReturnsBool::True;
                    }
                }
            }
        }
    }

    ReturnsBool::False
}

/// Checks the rules against the given method call.
pub fn try_rename_getter_call(method_call: &syn::ExprMethodCall) -> Result<RenameOk, RenameError> {
    let get_fn = GetFunction::try_from(method_call.method.to_string())?;

    if method_call.turbofish.is_some() {
        return Err(RenameError::GenericParams(get_fn.into_name()));
    }

    if !method_call.args.is_empty() {
        return Err(RenameError::MultipleArgs(get_fn.into_name()));
    }

    get_fn.try_rename(|| ReturnsBool::Maybe)
}

/// Get function suffix renaming successfull Result.
#[derive(Debug)]
#[non_exhaustive]
pub enum RenameOk {
    /// Fixing name to comply to rules. Ex. `get_active` -> `is_active`.
    Fix { name: String, new_name: String },
    /// Applying substitution. Ex. `get_mute` -> `is_muted`.
    Substitute { name: String, new_name: String },
    /// Regaular removal of the prefix.
    Regular { name: String, new_name: String },
}

impl RenameOk {
    /// Returns the new name.
    pub fn new_name(&self) -> &str {
        match self {
            RenameOk::Fix { new_name, .. }
            | RenameOk::Substitute { new_name, .. }
            | RenameOk::Regular { new_name, .. } => new_name.as_str(),
        }
    }

    /// Returns the original name.
    pub fn name(&self) -> &str {
        match self {
            RenameOk::Fix { name, .. }
            | RenameOk::Substitute { name, .. }
            | RenameOk::Regular { name, .. } => name.as_str(),
        }
    }

    pub fn is_fix(&self) -> bool {
        matches!(self, RenameOk::Fix{ .. })
    }

    pub fn is_substitute(&self) -> bool {
        matches!(self, RenameOk::Substitute{ .. })
    }
}

impl Display for RenameOk {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use RenameOk::*;

        match self {
            Fix { name, new_name } => write!(f, "fixing {} as {}", name, new_name),
            Substitute { name, new_name } => write!(f, "substituting {} with {}", name, new_name),
            Regular { name, new_name } => write!(f, "renaming {} as {}", name, new_name),
        }
    }
}

/// Get function suffix renaming failure Result.
#[derive(Debug)]
#[non_exhaustive]
pub enum RenameError {
    GenericParams(String),
    MultipleArgs(String),
    GetFunction(GetFunctionError),
    NoArgs(String),
    OneNoneSelfArg(String),
    Reserved(String),
}

impl Display for RenameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use RenameError::*;

        match self {
            GenericParams(name) => write!(f, "{}: generic parameter(s)", name),
            MultipleArgs(name) => write!(f, "{}: multiple arguments", name),
            GetFunction(err) => err.fmt(f),
            NoArgs(name) => write!(f, "{}: no arguments", name),
            OneNoneSelfArg(name) => write!(f, "{}: one none `self` argument", name),
            Reserved(name) => write!(f, "{}: name is reserved", name),
        }
    }
}

impl Error for RenameError {}

impl From<GetFunctionError> for RenameError {
    fn from(err: GetFunctionError) -> Self {
        RenameError::GetFunction(err)
    }
}

/// Suffix renaming failure Result.
#[derive(Debug)]
#[non_exhaustive]
pub struct GetFunctionError(String);

impl From<GetFunctionError> for String {
    fn from(err: GetFunctionError) -> Self {
        err.0
    }
}

impl Display for GetFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}: is not a get_* function", self.0)
    }
}

impl Error for GetFunctionError {}
