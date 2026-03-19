use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{Item, Type, parse_macro_input};

use crate::{
    functions::{expand_fn, expand_impl},
    structs::expand_struct,
};

mod functions;
mod structs;
mod type_resolver;

use type_resolver::FFITypeResolver;

#[proc_macro_attribute]
pub fn export(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_to_export = {
        let item = item.clone();
        parse_macro_input!(item as Item)
    };

    match item_to_export {
        Item::Struct(item) => expand_struct(item).into(),
        Item::Fn(_) => expand_fn(item.into()).into(),
        Item::Impl(_) => expand_impl(item.into()).into(),
        _ => unimplemented!(
            "#[ezffi::export] not supported item {}",
            quote! { #item_to_export }
        ),
    }
}

fn ffi_struct_name(name: &Ident) -> Ident {
    format_ident!("Ffi{}", name)
}

fn ffi_fn_name(fn_name: &Ident, impl_ty: Option<&Type>) -> Ident {
    if let Some(ty) = impl_ty {
        let ty_ident = match ty {
            syn::Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.clone(),
            _ => unimplemented!("Unsupported impl ty {}", quote!(#ty)),
        };

        format_ident!("ffi_{}_{}", ty_ident, fn_name)
    } else {
        format_ident!("ffi_{}", fn_name)
    }
}

fn ffi_free_fn_name(ty_name: &Ident) -> Ident {
    format_ident!("ffi_{}_free", ty_name)
}
