use quote::{format_ident, quote};
use syn::{Ident, Type};

pub struct FFINamer;

impl FFINamer {
    pub fn name_struct(rust_ty_name: &Ident) -> Ident {
        format_ident!("Ffi{}", rust_ty_name)
    }

    pub fn name_fn(rust_fn_name: &Ident, impl_ty: Option<&Type>) -> Ident {
        if let Some(ty) = impl_ty {
            let ty_ident = match ty {
                syn::Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.clone(),
                _ => unimplemented!("Unsupported impl ty {}", quote!(#ty)),
            };

            format_ident!("ffi_{}_{}", ty_ident, rust_fn_name)
        } else {
            format_ident!("ffi_{}", rust_fn_name)
        }
    }

    pub fn name_free_fn(rust_ty_name: &Ident) -> Ident {
        format_ident!("ffi_{}_free", rust_ty_name)
    }
}
