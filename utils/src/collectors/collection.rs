//! A collection of getters.

/// A `trait` that allows intergrating with [`Getter`](crate::Getter) Collections.
///
/// Parsing Rust code involves different components in order to
/// deal with regular code, macros and code in documentation. Implementing
/// this `trait` on data structure involved in collecting [`Getter`](crate::Getter)s
/// eases the integration in these components.
///
/// Parsing the documentation code is like parsing a single Rust file:
/// it can invole `struct` definitions and implementations as well as
/// `macro`s. However, the code appears at a particular `offset` in the
/// actual Rust file.
///
/// See also [`TokenStreamGetterCollector`](crate::TokenStreamGetterCollector),
/// [`SyntaxTreeGetterCollector`](crate::SyntaxTreeGetterCollector) and
/// [`DocCodeGetterCollector`](crate::DocCodeGetterCollector).
pub trait GetterCollection {
    /// Clones a view on the provided shared collection.
    ///
    /// Any [`Getter`](crate::Getter) added to the resulting collection
    /// must also be visible to `this`.
    fn clone(this: &Self) -> Self;

    /// Disables the generation of the doc alias attribute.
    ///
    /// For any [`Getter`](crate::Getter) added via this view of the
    /// [`GetterCollection`], the doc alias attribute will not be generated,
    /// regardless of the position in the Rust code.
    ///
    /// This facilitates the renaming of [`Getter`](crate::Getter)s in
    /// documentation code.
    fn disable_doc_alias(&mut self);

    /// Returns the offset of this view in the whole Rust file.
    fn offset(&self) -> usize;

    /// Defines the offset for this view in the whole Rust file.
    fn set_offset(&mut self, offset: usize);
}
