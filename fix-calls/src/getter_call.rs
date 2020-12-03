use rules::ReturnsBool;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use utils::{parser::prelude::*, Getter, GetterError};

#[derive(Debug)]
pub struct GetterCallCollectionInner(HashMap<usize, Vec<Getter>>);

impl Default for GetterCallCollectionInner {
    fn default() -> Self {
        GetterCallCollectionInner(HashMap::new())
    }
}

#[derive(Debug, Default)]
pub struct GetterCallCollection {
    inner: Rc<RefCell<GetterCallCollectionInner>>,
    offset: usize,
}

impl GetterCollection for GetterCallCollection {
    fn clone(this: &Self) -> Self {
        GetterCallCollection {
            inner: Rc::clone(&this.inner),
            offset: this.offset,
        }
    }

    fn disable_doc_alias(&mut self) {}

    fn offset(&self) -> usize {
        self.offset
    }

    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
}

impl GetterCallCollection {
    pub fn try_new_getter(
        &self,
        name: String,
        returns_bool: impl Into<ReturnsBool> + Copy,
        line: usize,
    ) -> Result<Getter, GetterError> {
        Getter::try_new(name, returns_bool, line + self.offset)
    }

    pub fn add(&self, getter: Getter) {
        let mut inner = self.inner.borrow_mut();
        let getter_calls_same_line = inner.0.entry(getter.line).or_insert_with(Vec::new);

        getter_calls_same_line.push(getter);
    }

    pub fn get(&self, line_idx: usize) -> Option<Vec<Getter>> {
        self.inner.borrow_mut().0.remove(&(line_idx + 1)) // convert line idx to line_nb
    }

    pub fn is_empty(&self) -> bool {
        self.inner.borrow().0.is_empty()
    }
}
