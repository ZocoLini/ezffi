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

#[proc_macro]
pub fn export_as_identity(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ty = parse_macro_input!(input as Type);

    quote! {
        impl crate::IntoFfi<()> for #ty {
            type Ffi = #ty;

            unsafe fn owned_into_ffi(self) -> Self::Ffi {
                self
            }

            unsafe fn ref_into_ffi(&self) -> Self::Ffi {
                *self
            }
        }

        impl crate::IntoRust<#ty> for #ty {
            unsafe fn into_rust(&self) -> &#ty {
                self
            }

            unsafe fn into_rust_mut(&mut self) -> &mut #ty {
                self
            }

            unsafe fn into_rust_owned(self) -> #ty {
                self
            }
        }
    }
    .into()
}

#[proc_macro]
pub fn export_extern_type_generic(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = parse_macro_input!(input as syn::Path);

    let ident = &path.segments.last().unwrap().ident;

    let free_fn = format_ident!("ezffi_free_{}", ident);

    quote! {
        #[repr(C)]
        pub struct #ident {
            inner: *mut std::os::raw::c_void,
        }

        impl<T> crate::IntoFfi<T> for #path<T> {
            type Ffi = #ident;

            unsafe fn ref_into_ffi(&self) -> #ident {
                #ident {
                    inner: self as *const #path<T> as *mut std::os::raw::c_void,
                }
            }

            unsafe fn owned_into_ffi(self) -> #ident {
                #ident {
                    inner: Box::into_raw(Box::new(self)) as *mut std::os::raw::c_void,
                }
            }
        }

        impl<T> crate::IntoRust<T> for #ident {
            unsafe fn into_rust(&self) -> &T {
                &*(self.inner as *const T)
            }

            unsafe fn into_rust_mut(&mut self) -> &mut T {
                &mut *(self.inner as *mut T)
            }

            unsafe fn into_rust_owned(self) -> T {
                std::ptr::read(self.inner as *const T)
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #free_fn(o: #ident) {
            let _ = Box::from_raw(o.inner as *mut #path<()>);
        }
    }
    .into()
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
