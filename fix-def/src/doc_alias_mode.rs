//! `doc-alias` attributes generation mode.

/// Mode to be used for `doc-alias` attributes generation.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DocAliasMode {
    /// Don't generate `doc-alias` attributes.
    Discard,
    /// Generate.
    Generate,
}

impl DocAliasMode {
    pub fn must_generate(self) -> bool {
        matches!(self, DocAliasMode::Generate)
    }
}
