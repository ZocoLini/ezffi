use syn::{DeriveInput, ItemStruct};

use crate::{FFINamer, GenerationType};
use quote::quote;

pub fn expand_struct(
    item: ItemStruct,
    generation_type: GenerationType,
) -> proc_macro2::TokenStream {
    let input = quote! { #item };
    let input: DeriveInput = syn::parse2(input).expect("Must be valid code");
    let ty_name = &input.ident;

    let ffi_name = FFINamer::name_struct(ty_name);
    let free_fn_name = FFINamer::name_free_fn(ty_name);

    super::FFITypeResolver::insert(&ty_name.to_string(), &ffi_name.to_string());

    let trait_location = match generation_type {
        GenerationType::Internal => quote! { crate },
        GenerationType::External => quote! { ezffi },
    };

    // Per-monomorphization drop fn so generic types deallocate with the right
    // Layout. The macro can't know the concrete T at expansion time, so the
    // FFI struct carries a function pointer captured by `owned_into_ffi`.
    let (impl_ffi_header, drop_fn_def, drop_fn_ref) = if item.generics.gt_token.is_some() {
        (
            quote! { impl<T> #trait_location::IntoFfi<T> for #ty_name<T> },
            quote! {
                unsafe extern "C" fn __ezffi_drop<T>(p: *mut core::ffi::c_void) {
                    drop(unsafe { Box::from_raw(p as *mut #ty_name<T>) });
                }
            },
            quote! { __ezffi_drop::<T> },
        )
    } else {
        (
            quote! { impl #trait_location::IntoFfi<()> for #ty_name },
            quote! {
                unsafe extern "C" fn __ezffi_drop(p: *mut core::ffi::c_void) {
                    drop(unsafe { Box::from_raw(p as *mut #ty_name) });
                }
            },
            quote! { __ezffi_drop },
        )
    };

    quote! {
        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct #ffi_name {
            inner: *mut core::ffi::c_void,
            drop_fn: unsafe extern "C" fn(*mut core::ffi::c_void),
            state: u8,
        }

        const _: () = {
            #drop_fn_def

            #impl_ffi_header {
                type Ffi = #ffi_name;

                unsafe fn ref_into_ffi(&self) -> Self::Ffi {
                    #ffi_name {
                        inner: self as *const Self as *mut core::ffi::c_void,
                        drop_fn: #drop_fn_ref,
                        state: #trait_location::TypeState::Ref as u8,
                    }
                }
                unsafe fn owned_into_ffi(self) -> Self::Ffi {
                    #ffi_name {
                        inner: Box::into_raw(Box::new(self)) as *mut core::ffi::c_void,
                        drop_fn: #drop_fn_ref,
                        state: #trait_location::TypeState::Owned as u8,
                    }
                }
            }
        };

        impl<T> #trait_location::IntoRust<T> for #ffi_name {
            unsafe fn into_rust(&self) -> &T {
                if self.state == #trait_location::TypeState::Freed as u8 {
                    panic!("Cannot borrow freed object");
                }

                unsafe { &*(self.inner as *mut T) }
            }

            unsafe fn into_rust_mut(&mut self) -> &mut T {
                if self.state == #trait_location::TypeState::Freed as u8 {
                    panic!("Cannot borrow freed object");
                }

                unsafe { &mut *(self.inner as *mut T) }
            }

            unsafe fn into_rust_owned(mut self) -> T {
                if self.state == #trait_location::TypeState::Freed as u8 {
                    panic!("Cannot own freed object");
                }

                if self.state == #trait_location::TypeState::Ref as u8 {
                    panic!("Cannot own an objects created from a reference");
                }

                let result = unsafe { *Box::from_raw(self.inner as *mut T) };
                self.state = #trait_location::TypeState::Freed as u8;
                result
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #free_fn_name(o: *const #ffi_name) {
            let mut o = unsafe { &mut *(o as *mut #ffi_name) };
            if o.state == #trait_location::TypeState::Freed as u8 {
                panic!("Cannot free freed object");
            }

            if o.state == #trait_location::TypeState::Ref as u8 {
                panic!("Cannot free objects created from a reference");
            }

            unsafe { (o.drop_fn)(o.inner); }
            o.state = #trait_location::TypeState::Freed as u8;
        }
    }
}
