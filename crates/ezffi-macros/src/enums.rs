use quote::quote;
use syn::ItemEnum;

use crate::{FFINamer, GenerationType, structs::expand_type};

pub fn is_c_compatible_enum(item: &ItemEnum) -> bool {
    item.variants
        .iter()
        .all(|v| matches!(v.fields, syn::Fields::Unit))
}

pub fn expand_enum(item: &ItemEnum, generation_type: GenerationType) -> proc_macro2::TokenStream {
    if item.generics.gt_token.is_some() {
        panic!("generic enums are not supported by #[ezffi::export]");
    } else {
        // Enums no C-compatibles are wrapped in a struct behind *mut c_void, the same
        // way no C-compatible structs are exposed
        expand_type(&item.ident, generation_type)
    }
}

pub fn expand_c_enum(item: &ItemEnum, generation_type: GenerationType) -> proc_macro2::TokenStream {
    let user_name = &item.ident;
    let ffi_name = FFINamer::name_struct(user_name);
    let variants = &item.variants;
    let attrs = &item.attrs;

    super::FFITypeResolver::insert(&user_name.to_string(), &ffi_name.to_string());

    let trait_location = match generation_type {
        GenerationType::Internal => quote! { crate },
        GenerationType::External => quote! { ezffi },
    };

    let has_repr = attrs.iter().any(|a| a.path().is_ident("repr"));
    let repr = if has_repr {
        quote! {}
    } else {
        quote! { #[repr(C)] }
    };

    quote! {
        #[derive(Clone, Copy)]
        #repr
        #(#attrs)*
        pub enum #ffi_name {
            #variants
        }

        pub type #user_name = #ffi_name;

        impl #trait_location::IntoFfi<()> for #ffi_name {
            type Ffi = #ffi_name;

            unsafe fn ref_into_ffi(&self) -> Self::Ffi { *self }
            unsafe fn owned_into_ffi(self) -> Self::Ffi { self }
        }

        impl #trait_location::IntoRust<#ffi_name> for #ffi_name {
            unsafe fn into_rust(&self) -> &#ffi_name { self }
            unsafe fn into_rust_mut(&mut self) -> &mut #ffi_name { self }
            unsafe fn into_rust_owned(self) -> #ffi_name { self }
        }
    }
}
