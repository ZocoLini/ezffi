use syn::{Generics, Ident, ItemEnum, ItemStruct};

use crate::{FFINamer, GenerationType};
use quote::quote;

pub fn expand_struct(
    item: &ItemStruct,
    generation_type: GenerationType,
) -> proc_macro2::TokenStream {
    expand_type(&item.ident, &item.generics, generation_type)
}

pub fn expand_enum(item: &ItemEnum, generation_type: GenerationType) -> proc_macro2::TokenStream {
    expand_type(&item.ident, &item.generics, generation_type)
}

/// Variants are all unit → emit a real C enum (renamed with FFI prefix) plus
/// an alias and identity `IntoFfi`/`IntoRust` impls so the standard wrapper
/// machinery (which always uses pointer-passing and trait method calls) works
/// without special-casing.
pub fn expand_c_enum(item: &ItemEnum, generation_type: GenerationType) -> proc_macro2::TokenStream {
    let user_name = &item.ident;
    let ffi_name = FFINamer::name_struct(user_name);
    let vis = &item.vis;
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
        #vis enum #ffi_name {
            #variants
        }

        #vis type #user_name = #ffi_name;

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

fn expand_type(
    ty_name: &Ident,
    generics: &Generics,
    generation_type: GenerationType,
) -> proc_macro2::TokenStream {
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
    let (impl_ffi_header, drop_fn_def, drop_fn_ref) = if generics.gt_token.is_some() {
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
            #[cfg(debug_assertions)]
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
                        #[cfg(debug_assertions)]
                        state: #trait_location::TypeState::Ref as u8,
                    }
                }
                unsafe fn owned_into_ffi(self) -> Self::Ffi {
                    #ffi_name {
                        inner: Box::into_raw(Box::new(self)) as *mut core::ffi::c_void,
                        drop_fn: #drop_fn_ref,
                        #[cfg(debug_assertions)]
                        state: #trait_location::TypeState::Owned as u8,
                    }
                }
            }
        };

        impl<T> #trait_location::IntoRust<T> for #ffi_name {
            unsafe fn into_rust(&self) -> &T {
                #[cfg(debug_assertions)]
                if self.state == #trait_location::TypeState::Freed as u8 {
                    panic!("Cannot borrow freed object");
                }

                unsafe { &*(self.inner as *mut T) }
            }

            unsafe fn into_rust_mut(&mut self) -> &mut T {
                #[cfg(debug_assertions)]
                if self.state == #trait_location::TypeState::Freed as u8 {
                    panic!("Cannot borrow freed object");
                }

                unsafe { &mut *(self.inner as *mut T) }
            }

            unsafe fn into_rust_owned(mut self) -> T {
                #[cfg(debug_assertions)]
                if self.state == #trait_location::TypeState::Freed as u8 {
                    panic!("Cannot own freed object");
                }

                #[cfg(debug_assertions)]
                if self.state == #trait_location::TypeState::Ref as u8 {
                    panic!("Cannot own an objects created from a reference");
                }

                let result = unsafe { *Box::from_raw(self.inner as *mut T) };
                #[cfg(debug_assertions)]
                { self.state = #trait_location::TypeState::Freed as u8; }
                result
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #free_fn_name(o: *const #ffi_name) {
            let mut o = unsafe { &mut *(o as *mut #ffi_name) };
            #[cfg(debug_assertions)]
            if o.state == #trait_location::TypeState::Freed as u8 {
                panic!("Cannot free freed object");
            }

            #[cfg(debug_assertions)]
            if o.state == #trait_location::TypeState::Ref as u8 {
                panic!("Cannot free objects created from a reference");
            }

            unsafe { (o.drop_fn)(o.inner); }
            #[cfg(debug_assertions)]
            { o.state = #trait_location::TypeState::Freed as u8; }
        }
    }
}
