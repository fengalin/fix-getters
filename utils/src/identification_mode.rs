//! Getter identification mode.

/// Mode to be used for getter identification.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IdentificationMode {
    /// Apply conservative rules:
    /// * The function is a method. This excludes standalone functions or associated
    ///   functions.
    /// * The function accepts no arguments besides `&['_][mut] self`. The methods which
    ///   accept other arguments are not `getter`s in the sense that they usually don't
    ///   return current value of a field and renaming them would harm the semantic.
    ///   Functions consuming `self` were also considered not eligible for renaming.
    /// * The function accepts no type parameter (lifetimes are accepted). The reason is
    ///   the same as for functions accepting multiple arguments (see above).
    Conservative,
    /// Apply name rules to all function with the `get` prefix.
    AllGetFunctions,
}

impl IdentificationMode {
    pub fn is_conservative(self) -> bool {
        matches!(self, IdentificationMode::Conservative)
    }
}
