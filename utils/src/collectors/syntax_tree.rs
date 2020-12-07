//! A [`Getter`](crate::Getter)s collector visting a [`SyntaxTree`](syn::File).

use crate::GetterCollection;
use std::path::Path;

/// A [`Getter`](crate::Getter)s collector visting a [`SyntaxTree`](syn::File).
///
/// A [`SyntaxTree`](syn::File) is obtained by parsing a code source with [`syn`].
/// They contain more syntatical information than a [`TokenStream`](syn::TokenStream),
/// however, they imply the syntax is valid. For macros, use a
/// [`TokenStreamGetterCollector`](super::TokenStreamGetterCollector) and
/// for documentation code, use a [`DocCodeGetterCollector`](super::DocCodeGetterCollector).
pub trait SyntaxTreeGetterCollector: for<'ast> syn::visit::Visit<'ast> {
    /// Type for the [`GetterCollection`] used by this [`GetterVisitor`].
    type GetterCollection: GetterCollection;

    /// Visits the `syntax_tree` collecting [`Getter`](crate::Getter)s
    /// in the [`GetterCollection`].
    fn collect(path: &Path, syntax_tree: &syn::File, getter_collection: &Self::GetterCollection);
}
