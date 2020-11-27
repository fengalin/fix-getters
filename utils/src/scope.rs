use std::{cell::RefCell, fmt, rc::Rc, string::ToString};

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
            // FIXME
            let type_ident = format_type_name(self_ty);

            if let Some((_, trait_path, _)) = trait_ {
                let trait_ident = path_ident(&trait_path);

                Scope::TraitImpl {
                    trait_: trait_ident,
                    type_: type_ident,
                }
            } else {
                Scope::StructImpl(type_ident)
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

fn path_ident(path: &syn::Path) -> String {
    if path.segments.is_empty() {
        return String::default();
    }

    let syn::PathSegment { ident, .. } = &path.segments.last().unwrap();
    ident.to_string()
}

fn format_type_name(self_ty: &syn::Type) -> String {
    match self_ty {
        syn::Type::Path(syn::TypePath { path, .. }) => path_ident(&path),
        syn::Type::Reference(syn::TypeReference {
            lifetime,
            mutability,
            elem,
            ..
        }) => {
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

            format!("{}{}", prefix, format_type_name(&elem))
        }
        syn::Type::Slice(syn::TypeSlice { elem, .. }) => format!("[{}]", format_type_name(&elem)),
        syn::Type::TraitObject(syn::TypeTraitObject { bounds, .. }) => {
            let mut trait_bound = "dyn ".to_string();
            for (idx, bound) in bounds.into_iter().enumerate() {
                if idx > 0 {
                    trait_bound += " + ";
                }

                if let syn::TypeParamBound::Trait(trait_) = bound {
                    trait_bound += &path_ident(&trait_.path);
                }
            }
            trait_bound
        }
        syn::Type::Tuple(syn::TypeTuple { elems, .. }) => {
            let mut tuple = "(".to_string();
            for (idx, elem) in elems.into_iter().enumerate() {
                if idx > 0 {
                    tuple += ", ";
                }

                tuple += &format_type_name(elem);
            }
            tuple + ")"
        }
        syn::Type::Paren(syn::TypeParen { elem, .. }) => {
            format!("({})", format_type_name(&elem))
        }
        syn::Type::Ptr(syn::TypePtr { elem, .. }) => {
            format!("*{}", format_type_name(&elem))
        }
        _ => panic!("unexpected self_ty {:#?}", self_ty),
    }
}
