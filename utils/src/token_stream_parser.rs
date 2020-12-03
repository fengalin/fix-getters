//! A [`TokenStream`](proc_macro2::TokenStream) parser `trait`.

use std::{cell::RefCell, rc::Rc};

/// A [`TokenStream`](proc_macro2::TokenStream) parser `trait`.
use crate::{GetterCollection, Scope};

pub trait TokenStreamParser {
    /// Type for the [`GetterCollection`] used by this [`TokenStreamParser`].
    type GetterCollection: GetterCollection;

    /// Parses the `stream` collecting [`Getter`](crate::Getter)s
    /// in the [`GetterCollection`].
    fn parse(
        stream: &proc_macro2::TokenStream,
        scope: &Rc<RefCell<Scope>>,
        getter_collection: &Self::GetterCollection,
    );
}
