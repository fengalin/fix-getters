use rules::{NewName, ReturnsBool};
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display},
    path::Path,
    rc::Rc,
};
use utils::{parser::prelude::*, Getter, GetterError};

#[derive(Debug)]
pub struct GetterDef {
    getter: Getter,
    needs_doc_alias: bool,
}

impl GetterDef {
    fn try_new(
        name: String,
        returns_bool: impl Into<ReturnsBool> + Copy,
        line: usize,
    ) -> Result<Self, GetterError> {
        Getter::try_new(name, returns_bool, line).map(|getter| GetterDef {
            getter,
            needs_doc_alias: false,
        })
    }

    pub fn name(&self) -> &str {
        &self.getter.name
    }

    pub fn new_name(&self) -> &NewName {
        &self.getter.new_name
    }

    pub fn returns_bool(&self) -> ReturnsBool {
        self.getter.new_name.returns_bool()
    }

    pub fn set_returns_bool(&mut self, returns_bool: impl Into<ReturnsBool>) {
        self.getter.set_returns_bool(returns_bool);
    }

    pub fn line(&self) -> usize {
        self.getter.line
    }

    pub fn needs_doc_alias(&self) -> bool {
        self.needs_doc_alias
    }

    pub fn set_needs_doc_alias(&mut self, needs_doc_alias: bool) {
        self.needs_doc_alias = needs_doc_alias;
    }

    pub fn log(&self, path: &Path, scope: &dyn Display) {
        self.getter.log(path, scope);
    }
}

impl Display for GetterDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.getter,
            if self.needs_doc_alias {
                " needs doc alias"
            } else {
                ""
            },
        )
    }
}

#[derive(Debug)]
pub struct GetterDefCollectionInner(HashMap<usize, GetterDef>);

impl Default for GetterDefCollectionInner {
    fn default() -> Self {
        GetterDefCollectionInner(HashMap::new())
    }
}

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
