//! Rust code scope identification.

use std::{cell::RefCell, fmt, rc::Rc, string::ToString};

/// Rust code scope identification.
#[derive(Debug)]
pub enum Scope {
    Const(String),
    Fn(String),
    Macro(String),
    Static(String),
    StructImpl(String),
    Trait(String),
    TraitImpl { trait_: String, type_: String },
    Unexpected,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::Unexpected
    }
}

impl From<&syn::Item> for Scope {
    fn from(node: &syn::Item) -> Self {
        match node {
            syn::Item::Const(item) => Scope::Const(item.ident.to_string()),
            syn::Item::Fn(fn_) => Scope::Fn(fn_.sig.ident.to_string()),
            syn::Item::Impl(impl_) => {
                let type_ident = format_type_name(&impl_.self_ty);

                if let Some((_, trait_path, _)) = &impl_.trait_ {
                    let trait_ident = path_ident(&trait_path);

                    Scope::TraitImpl {
                        trait_: trait_ident,
                        type_: type_ident,
                    }
                } else {
                    Scope::StructImpl(type_ident)
                }
            }
            syn::Item::Macro(macro_) => Scope::Macro(
                macro_
                    .ident
                    .as_ref()
                    .map(|ident| ident.to_string())
                    .unwrap_or_else(|| "unnamed".to_string()),
            ),
            syn::Item::Macro2(macro2) => Scope::Macro(macro2.ident.to_string()),
            syn::Item::Static(static_) => Scope::Static(static_.ident.to_string()),
            syn::Item::Trait(trait_) => Scope::Trait(trait_.ident.to_string()),
            _ => Scope::Unexpected,
        }
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
            TraitImpl { trait_, type_ } => write!(f, "impl {} for {}", trait_, type_),
            Unexpected => f.write_str("**Unexpected scope**"),
        }
    }
}

fn path_ident(path: &syn::Path) -> String {
    if path.segments.is_empty() {
        return String::default();
    }
    path.segments.last().unwrap().ident.to_string()
}

fn format_type_name(self_ty: &syn::Type) -> String {
    match self_ty {
        syn::Type::Path(path) => path_ident(&path.path),
        syn::Type::Reference(ref_) => {
            let prefix = match &ref_.lifetime {
                None => if ref_.mutability.is_some() {
                    "&mut "
                } else {
                    "&"
                }
                .to_string(),
                Some(lifetime) => {
                    if ref_.mutability.is_some() {
                        format!("&{} mut ", lifetime.to_string())
                    } else {
                        format!("&{} ", lifetime.to_string())
                    }
                }
            };

            format!("{}{}", prefix, format_type_name(&ref_.elem))
        }
        syn::Type::Slice(slice) => format!("[{}]", format_type_name(&slice.elem)),
        syn::Type::TraitObject(trait_obj) => {
            let mut trait_bound = "dyn ".to_string();
            for (idx, bound) in trait_obj.bounds.iter().enumerate() {
                if idx > 0 {
                    trait_bound += " + ";
                }

                if let syn::TypeParamBound::Trait(trait_) = bound {
                    trait_bound += &path_ident(&trait_.path);
                }
            }
            trait_bound
        }
        syn::Type::Tuple(tuple) => {
            let mut tuple_str = "(".to_string();
            for (idx, elem) in tuple.elems.iter().enumerate() {
                if idx > 0 {
                    tuple_str += ", ";
                }

                tuple_str += &format_type_name(elem);
            }
            tuple_str + ")"
        }
        syn::Type::Paren(paren) => {
            format!("({})", format_type_name(&paren.elem))
        }
        syn::Type::Ptr(ptr) => {
            format!("*{}", format_type_name(&ptr.elem))
        }
        _ => unimplemented!("format type formatting for {:#?}", self_ty),
    }
}
