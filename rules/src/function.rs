use lazy_static::lazy_static;
use std::collections::HashSet;
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
        reserved.insert("main");
        reserved.insert("mut");
        reserved.insert("optional");
        reserved.insert("owned");
        reserved.insert("some");
        reserved.insert("type");
        reserved
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

/// Checks the rules against the given function signature.
pub fn check<'ast>(sig: &'ast syn::Signature) -> Result<CheckOk, CheckError> {
    use CheckError::*;

    let name = sig.ident.to_string();
    let new_name = name.strip_prefix("get_");

    let new_name = match new_name {
        Some(new_name) => new_name,
        None => return Err(NotAGetFn),
    };

    let syn::Generics { params, .. } = &sig.generics;
    if !params.is_empty() {
        return Err(GenericParams);
    }

    if sig.inputs.len() > 1 {
        return Err(MutlipleArgs);
    }

    match sig.inputs.first() {
        Some(syn::FnArg::Receiver { .. }) => (),
        Some(_) => return Err(OneNoneSelfArg),
        None => return Err(NoArgs),
    }

    if RESERVED.contains(new_name) {
        return Err(Reserved);
    }

    let splits: Vec<&str> = new_name.splitn(2, '_').collect();
    if splits.len() > 1 && PREFIX_TO_POSTFIX.contains(splits[0]) {
        Ok(CheckOk::Fixed(format!("{}_{}", splits[1], splits[0])))
    } else {
        Ok(CheckOk::Unchanged(new_name.to_string()))
    }
}

#[derive(Debug)]
pub enum CheckOk {
    Fixed(String),
    Unchanged(String),
}

impl CheckOk {
    pub fn into_inner(self) -> String {
        use CheckOk::*;
        match self {
            Fixed(inner) => inner,
            Unchanged(inner) => inner,
        }
    }

    pub fn is_fixed(&self) -> bool {
        matches!(self, CheckOk::Fixed(_))
    }
}

#[derive(Debug)]
pub enum CheckError {
    GenericParams,
    MutlipleArgs,
    NotAGetFn,
    NoArgs,
    OneNoneSelfArg,
    Reserved,
}

impl Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use CheckError::*;

        match self {
            GenericParams => f.write_str("generic parameters"),
            MutlipleArgs => f.write_str("multiple arguments"),
            NotAGetFn => f.write_str("not a get function"),
            NoArgs => f.write_str("no arguments"),
            OneNoneSelfArg => f.write_str("none `self` one argument"),
            Reserved => f.write_str("reserved"),
        }
    }
}

impl Error for CheckError {}
