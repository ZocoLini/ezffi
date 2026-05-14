use quote::{format_ident, quote};
use syn::{Ident, Type};

pub struct FFINamer;

impl FFINamer {
    pub fn name_struct(rust_ty_name: &Ident) -> Ident {
        let prefix = crate::CONFIG.type_prefix();
        let suffix = crate::CONFIG.type_suffix();

        let ffi_ty_name = format!("{}{}{}", prefix, rust_ty_name, suffix);
        let ffi_ty_name = crate::CONFIG.type_case_style().format(&ffi_ty_name);
        format_ident!("{}", ffi_ty_name)
    }

    pub fn name_fn(rust_fn_name: &Ident, impl_ty: Option<&Type>) -> Ident {
        let prefix = crate::CONFIG.fns_prefix();
        let suffix = crate::CONFIG.fns_suffix();

        let ffi_fn_name = if let Some(ty) = impl_ty {
            let ffi_ty_name = match ty {
                syn::Type::Path(type_path) => {
                    let rust_ty_name = type_path.path.segments.last().unwrap().ident.clone();
                    Self::name_struct(&rust_ty_name)
                }
                _ => unimplemented!("Unsupported impl ty {}", quote!(#ty)),
            };

            let ffi_fn_name = format!("{}{}_{}{}", prefix, ffi_ty_name, rust_fn_name, suffix);
            crate::CONFIG.fns_case_style().format(&ffi_fn_name)
        } else {
            let ffi_fn_name = format!("{}{}{}", prefix, rust_fn_name, suffix);
            crate::CONFIG.fns_case_style().format(&ffi_fn_name)
        };
        let ffi_fn_name = crate::CONFIG.fns_case_style().format(&ffi_fn_name);
        format_ident!("{}", ffi_fn_name)
    }

    pub fn name_free_fn(rust_ty_name: &Ident) -> Ident {
        let prefix = crate::CONFIG.free_fns_prefix();
        let suffix = crate::CONFIG.free_fns_suffix();

        let ffi_ty_name = Self::name_struct(rust_ty_name);

        let ffi_free_fn_name = format!("{}{}_free{}", prefix, ffi_ty_name, suffix);
        let ffi_free_fn_name = crate::CONFIG
            .free_fns_case_style()
            .format(&ffi_free_fn_name);
        format_ident!("{}", ffi_free_fn_name)
    }
}
