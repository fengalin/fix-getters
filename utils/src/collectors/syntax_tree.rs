//! A [`Visit`](syn::visit::Visit) `trait` dedicated at [`Getter`](crate::Getter)s collection.

use crate::GetterCollection;
use std::path::Path;

/// A [`Visit`](syn::visit::Visit) `trait` dedicated at [`Getter`](crate::Getter)s collection.
pub trait GetterVisitor: for<'ast> syn::visit::Visit<'ast> {
    /// Type for the [`GetterCollection`] used by this [`GetterVisitor`].
    type GetterCollection: GetterCollection;

    /// Visits the `syntax_tree` collecting [`Getter`](crate::Getter)s
    /// in the [`GetterCollection`].
    fn visit(path: &Path, syntax_tree: &syn::File, getter_collection: &Self::GetterCollection);
}
