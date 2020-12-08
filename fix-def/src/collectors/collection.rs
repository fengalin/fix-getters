//! A collection of [`GetterDef`](crate::GetterDef)s.

use rules::ReturnsBool;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use utils::{prelude::*, GetterError};

use crate::GetterDef;

#[derive(Debug)]
struct GetterDefCollectionInner(HashMap<usize, GetterDef>);

impl Default for GetterDefCollectionInner {
    fn default() -> Self {
        GetterDefCollectionInner(HashMap::new())
    }
}

/// A collection of [`GetterDef`](crate::GetterDef)s.
///
/// Manages [`GetterDef`](crate::GetterDef)s which were considered
/// eligibles to be renamed.
#[derive(Debug, Default)]
pub struct GetterDefCollection {
    inner: Rc<RefCell<GetterDefCollectionInner>>,
    offset: usize,
    blocks_doc_alias: bool,
}

impl GetterCollection for GetterDefCollection {
    fn clone(this: &Self) -> Self {
        GetterDefCollection {
            inner: Rc::clone(&this.inner),
            offset: this.offset,
            blocks_doc_alias: this.blocks_doc_alias,
        }
    }

    fn disable_doc_alias(&mut self) {
        self.blocks_doc_alias = true;
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
}

impl GetterDefCollection {
    pub fn try_new_getter(
        &self,
        name: String,
        returns_bool: impl Into<ReturnsBool> + Copy,
        line: usize,
    ) -> Result<GetterDef, GetterError> {
        GetterDef::try_new(name, returns_bool, line + self.offset)
    }

    pub fn add(&self, getter_def: GetterDef) {
        let line_idx = getter_def.line();
        if self
            .inner
            .borrow_mut()
            .0
            .insert(line_idx, getter_def)
            .is_some()
        {
            panic!("Found more than one getter definition @ {}", line_idx + 1);
        }
    }

    pub fn get(&self, line_idx: usize) -> Option<GetterDef> {
        self.inner.borrow_mut().0.remove(&(line_idx + 1)) // convert line idx to line_nb
    }

    pub fn is_empty(&self) -> bool {
        self.inner.borrow().0.is_empty()
    }
}
