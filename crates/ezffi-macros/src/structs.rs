use syn::{DeriveInput, ItemStruct};

use crate::{ffi_free_fn_name, ffi_struct_name};
use quote::quote;

pub fn expand_struct(item: ItemStruct) -> proc_macro2::TokenStream {
    let input = quote! { #item };
    let input: DeriveInput = syn::parse2(input).expect("Must be valid code");
    let ty_name = &input.ident;

    let ffi_name = ffi_struct_name(ty_name);
    let free_fn_name = ffi_free_fn_name(ty_name);

    super::FFITypeResolver::insert(&ty_name.to_string(), &ffi_name.to_string());

    let (impl_ffi_header, free_converter) = if item.generics.gt_token.is_some() {
        (
            quote! { impl<T> ezffi::IntoFfi<T> for #ty_name<T> },
            quote! { *mut #ty_name<()> },
        )
    } else {
        (
            quote! { impl ezffi::IntoFfi<()> for #ty_name },
            quote! { *mut #ty_name },
        )
    };

    quote! {
        #input

        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct #ffi_name {
            inner: *mut core::ffi::c_void,
        }

        #impl_ffi_header {
            type Ffi = #ffi_name;

            unsafe fn ref_into_ffi(&self) -> Self::Ffi {
                #ffi_name {
                    inner: self as *const Self as *mut core::ffi::c_void,
                }
            }
            unsafe fn owned_into_ffi(self) -> Self::Ffi {
                #ffi_name {
                    inner: Box::into_raw(Box::new(self)) as *mut core::ffi::c_void,
                }
            }
        }

        impl<T> ezffi::IntoRust<T> for #ffi_name {
            unsafe fn into_rust(&self) -> &T {
                unsafe { &*(self.inner as *mut T) }
            }

            unsafe fn into_rust_mut(&mut self) -> &mut T {
                unsafe { &mut *(self.inner as *mut T) }
            }

            unsafe fn into_rust_owned(self) -> T {
                unsafe { *Box::from_raw(self.inner as *mut T) }
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #free_fn_name(o: #ffi_name) {
            let _ = unsafe { Box::from_raw(o.inner as #free_converter) };
        }
    }
}
