use syn::{Generics, Ident, ItemEnum, ItemStruct};

use crate::{FFINamer, GenerationType};
use quote::quote;

pub fn expand_struct(
    item: &ItemStruct,
    generation_type: GenerationType,
) -> proc_macro2::TokenStream {
    if item.generics.gt_token.is_some() {
        expand_generic_type(&item.ident, &item.generics, generation_type)
    } else {
        expand_type(&item.ident, generation_type)
    }
}

pub fn expand_enum(item: &ItemEnum, generation_type: GenerationType) -> proc_macro2::TokenStream {
    if item.generics.gt_token.is_some() {
        panic!("generic enums are not supported by #[ezffi::export]");
    }
    expand_type(&item.ident, generation_type)
}

pub fn is_c_compatible_struct(item: &ItemStruct) -> bool {
    for field in &item.fields {
        if !super::FFITypeResolver::is_c_compatible(&field.ty) {
            return false;
        }
    }

    !item.fields.is_empty() // No-fields structs are not c-compatible (for now)
}

pub fn expand_c_struct(
    item: &ItemStruct,
    generation_type: GenerationType,
) -> proc_macro2::TokenStream {
    let user_name = &item.ident;
    let ffi_name = FFINamer::name_struct(user_name);
    let fields = &item.fields;
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

    // Named fields render as `{ ... }` (no semicolon); tuple fields as
    // `( ... )` and need a trailing `;`.
    let semi = match fields {
        syn::Fields::Named(_) => quote! {},
        _ => quote! { ; },
    };

    quote! {
        #[derive(Clone, Copy)]
        #repr
        #(#attrs)*
        pub struct #ffi_name #fields #semi

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

fn expand_type(ty_name: &Ident, generation_type: GenerationType) -> proc_macro2::TokenStream {
    let ffi_name = FFINamer::name_struct(ty_name);
    let free_fn_name = FFINamer::name_free_fn(ty_name);

    super::FFITypeResolver::insert(&ty_name.to_string(), &ffi_name.to_string());

    let trait_location = match generation_type {
        GenerationType::Internal => quote! { crate },
        GenerationType::External => quote! { ezffi },
    };

    quote! {
        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct #ffi_name {
            inner: *mut core::ffi::c_void,
            #[cfg(debug_assertions)]
            state: u8,
        }

        const _: () = {
            impl #trait_location::IntoFfi<()> for #ty_name {
                type Ffi = #ffi_name;

                unsafe fn ref_into_ffi(&self) -> Self::Ffi {
                    #ffi_name {
                        inner: self as *const Self as *mut core::ffi::c_void,
                        #[cfg(debug_assertions)]
                        state: #trait_location::TypeState::Ref as u8,
                    }
                }
                unsafe fn owned_into_ffi(self) -> Self::Ffi {
                    #ffi_name {
                        inner: Box::into_raw(Box::new(self)) as *mut core::ffi::c_void,
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

            unsafe { drop(Box::from_raw(o.inner as *mut #ty_name)); }
            #[cfg(debug_assertions)]
            { o.state = #trait_location::TypeState::Freed as u8; }
        }
    }
}

#[cfg(feature = "generics")]
fn expand_generic_type(
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

    if generics.lifetimes().next().is_some() {
        panic!("lifetime parameters are not supported by #[ezffi::export]");
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let turbofish = ty_generics.as_turbofish();

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
            // Per-monomorphization drop, captured as a fn pointer at
            // construction time so the free fn deallocates with the right Layout.
            unsafe extern "C" fn __ezffi_drop #impl_generics (
                p: *mut core::ffi::c_void,
            ) #where_clause {
                let _ = unsafe { Box::from_raw(p as *mut #ty_name #ty_generics) };
            }

            impl #impl_generics #trait_location::IntoFfi<()>
                for #ty_name #ty_generics #where_clause
            {
                type Ffi = #ffi_name;

                unsafe fn ref_into_ffi(&self) -> Self::Ffi {
                    #ffi_name {
                        inner: self as *const Self as *mut core::ffi::c_void,
                        drop_fn: __ezffi_drop #turbofish,
                        #[cfg(debug_assertions)]
                        state: #trait_location::TypeState::Ref as u8,
                    }
                }
                unsafe fn owned_into_ffi(self) -> Self::Ffi {
                    #ffi_name {
                        inner: Box::into_raw(Box::new(self)) as *mut core::ffi::c_void,
                        drop_fn: __ezffi_drop #turbofish,
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

#[cfg(not(feature = "generics"))]
fn expand_generic_type(
    _ty_name: &Ident,
    _generics: &Generics,
    _generation_type: GenerationType,
) -> proc_macro2::TokenStream {
    panic!("generic types require enabling the `generics` feature on ezffi",);
}
