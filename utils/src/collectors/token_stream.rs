//! A [`TokenStream`](proc_macro2::TokenStream) parser `trait`.

use std::path::Path;

/// A [`TokenStream`](proc_macro2::TokenStream) parser `trait`.
use crate::{GetterCollection, Scope};

pub trait TokenStreamParser {
    /// Type for the [`GetterCollection`] used by this [`TokenStreamParser`].
    type GetterCollection: GetterCollection;

    /// Parses the `stream` collecting [`Getter`](crate::Getter)s
    /// in the [`GetterCollection`].
    fn parse(
        path: &Path,
        scope: &Scope,
        stream: &proc_macro2::TokenStream,
        getter_collection: &Self::GetterCollection,
    );
}
