//! A [`Getter`](crate::Getter)s collector visting a [`TokenStream`](proc_macro2::TokenStream).

use std::path::Path;

use crate::{GetterCollection, Scope};

/// A [`Getter`](crate::Getter)s collector visting a [`TokenStream`](proc_macro2::TokenStream).
///
/// A [`TokenStream`](proc_macro2::TokenStream) is provided by [`syn`] when a macro
/// or is encountered.
///
/// This is also useful to parse documentation, once the different lines of the documentation
/// and the code it contains is gethered together. Use a [`DocCodeGetterCollector`](super::DocCodeGetterCollector)
/// for that.
///
/// For regular Rust code, it is easier to work on a [`SyntaxTree`](syn::File) using
/// a [`SyntaxTreeGetterCollector`](crate::DocCodeGetterCollector).
pub trait TokenStreamGetterCollector {
    /// Type for the [`GetterCollection`] used by this [`TokenStreamGetterCollector`].
    type GetterCollection: GetterCollection;

    /// Parses the [`TokenStream`](proc_macro2::TokenStream) collecting [`Getter`](crate::Getter)s
    /// in the [`GetterCollection`].
    fn collect(
        path: &Path,
        scope: &Scope,
        stream: &proc_macro2::TokenStream,
        getter_collection: &Self::GetterCollection,
    );
}
