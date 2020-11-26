use std::{
    cell::{Ref, RefCell},
    fmt,
    rc::Rc,
    string::ToString,
};

#[derive(Debug)]
pub enum Scope {
    Fn,
    StructImpl(String),
    Trait(String),
    TraitImpl { trait_: String, struct_: String },
    Unexpected,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::Unexpected
    }
}

impl From<Scope> for Rc<RefCell<Scope>> {
    fn from(scope: Scope) -> Self {
        Rc::new(RefCell::new(scope))
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use Scope::*;

        match self {
            Fn => f.write_str("fn _"),
            StructImpl(struct_) => f.write_str(struct_),
            Trait(trait_) => f.write_str(trait_),
            TraitImpl { trait_, struct_ } => write!(f, "impl {} for {}", trait_, struct_),
            Unexpected => f.write_str("**Unexpected**"),
        }
    }
}

#[derive(Debug)]
pub struct FnWithScope {
    fn_: String,
    scope: Rc<RefCell<Scope>>,
}

impl FnWithScope {
    pub fn new(fn_: impl ToString, scope: &Rc<RefCell<Scope>>) -> Self {
        FnWithScope {
            fn_: fn_.to_string(),
            scope: Rc::clone(scope),
        }
    }

    pub fn fn_(&self) -> &str {
        &self.fn_
    }

    pub fn scope(&self) -> Ref<Scope> {
        self.scope.borrow()
    }
}

impl fmt::Display for FnWithScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use Scope::*;

        match &*self.scope.borrow() {
            StructImpl(struct_) => write!(f, "{}::{}", struct_, self.fn_),
            Trait(trait_) => write!(f, "{}::{}", trait_, self.fn_),
            TraitImpl { trait_, struct_ } => {
                write!(f, "{}::{} impl for {}", trait_, self.fn_, struct_)
            }
            other => Scope::fmt(other, f),
        }
    }
}

pub fn item_scope(node: &syn::Item) -> Scope {
    match node {
        syn::Item::Fn(_) => Scope::Fn,
        syn::Item::Impl(syn::ItemImpl {
            self_ty, trait_, ..
        }) => {
            let struct_ident = match self_ty.as_ref() {
                syn::Type::Path(syn::TypePath { path, .. }) => path_ident(&self_ty, &path),
                syn::Type::Reference(syn::TypeReference {
                    lifetime,
                    mutability,
                    elem,
                    ..
                }) => match elem.as_ref() {
                    syn::Type::Path(syn::TypePath { path, .. }) => {
                        let prefix = match lifetime {
                            None => if mutability.is_some() { "&mut " } else { "&" }.to_string(),
                            Some(lifetime) => {
                                if mutability.is_some() {
                                    format!("&{} mut ", lifetime.to_string())
                                } else {
                                    format!("&{} ", lifetime.to_string())
                                }
                            }
                        };

                        format!("{}{}", prefix, path_ident(&self_ty, &path))
                    }
                    _ => panic!("unexpected Reference elem in self_ty {:#?}", self_ty),
                },
                // FIXME parse the tuple... or not :)
                syn::Type::Tuple(..) => "(_, ...)".to_string(),
                _ => panic!("unexpected self_ty {:#?}", self_ty),
            };

            if let Some((_, trait_path, _)) = trait_ {
                let trait_ident = path_ident(&self_ty, &trait_path);

                Scope::TraitImpl {
                    trait_: trait_ident,
                    struct_: struct_ident,
                }
            } else {
                Scope::StructImpl(struct_ident)
            }
        }
        syn::Item::Trait(syn::ItemTrait { ident, .. }) => Scope::Trait(ident.to_string()),
        _ => Scope::Unexpected,
    }
}

fn path_ident(self_ty: &syn::Type, path: &syn::Path) -> String {
    if path.segments.is_empty() {
        panic!("no segments in path for self_ty {:#?}", self_ty);
    }

    // FIXME use more that just the last segment
    let syn::PathSegment { ident, .. } = &path.segments.last().unwrap();

    ident.to_string()
}
