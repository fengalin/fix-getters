use std::{cell::RefCell, fmt, rc::Rc, string::ToString};

#[derive(Debug)]
pub enum Scope {
    Const(String),
    Fn(String),
    Macro(String),
    Static(String),
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
            Const(name) => write!(f, "const {}", name),
            Fn(name) => write!(f, "fn {}", name),
            Macro(name) => write!(f, "macro! {}", name),
            Static(name) => write!(f, "static {}", name),
            StructImpl(struct_) => f.write_str(struct_),
            Trait(trait_) => f.write_str(trait_),
            TraitImpl { trait_, struct_ } => write!(f, "impl {} for {}", trait_, struct_),
            Unexpected => f.write_str("**Unexpected**"),
        }
    }
}

pub fn item_scope(node: &syn::Item) -> Scope {
    match node {
        syn::Item::Const(syn::ItemConst { ident, .. }) => Scope::Const(ident.to_string()),
        syn::Item::Fn(syn::ItemFn { sig, .. }) => Scope::Fn(sig.ident.to_string()),
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
        syn::Item::Macro(syn::ItemMacro { ident, .. }) => Scope::Macro(
            ident
                .as_ref()
                .map(|ident| ident.to_string())
                .unwrap_or_else(|| "unnamed".to_string()),
        ),
        syn::Item::Macro2(syn::ItemMacro2 { ident, .. }) => Scope::Macro(ident.to_string()),
        syn::Item::Static(syn::ItemStatic { ident, .. }) => Scope::Static(ident.to_string()),
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
