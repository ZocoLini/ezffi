use syn::{DeriveInput, ItemStruct};

use crate::{ffi_free_fn_name, ffi_struct_name};
use quote::quote;

pub fn expand_struct(item: ItemStruct) -> proc_macro2::TokenStream {
    if item.generics.gt_token.is_some() {
        expand_generic_struct(quote! { #item })
    } else {
        expand_simple_struct(quote! { #item })
    }
}

fn expand_simple_struct(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let input: DeriveInput = syn::parse2(input).expect("Must be valid code");
    let ty_name = &input.ident;

    let ffi_name = ffi_struct_name(ty_name);
    let free_fn_name = ffi_free_fn_name(ty_name);

    super::FFITypeResolver::insert(&ty_name.to_string(), &ffi_name.to_string());

    quote! {
        #input

        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct #ffi_name {
            inner: *mut core::ffi::c_void,
        }

        impl ezffi::IntoFfi for #ty_name {
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

        impl ezffi::IntoRust for #ffi_name {
            type Rust = #ty_name;

            unsafe fn into_rust(&self) -> &Self::Rust {
                unsafe { &*(self.inner as *mut Self::Rust) }
            }

            unsafe fn into_rust_mut(&mut self) -> &mut Self::Rust {
                unsafe { &mut *(self.inner as *mut Self::Rust) }
            }

            unsafe fn into_rust_owned(self) -> Self::Rust {
                unsafe { *Box::from_raw(self.inner as *mut Self::Rust) }
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #free_fn_name(o: #ffi_name) {
            let _ = unsafe { ezffi::IntoRust::into_rust_owned(o) };
        }
    }
}

fn expand_generic_struct(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let input: DeriveInput = syn::parse2(input).expect("Must be valid code");
    let ty_name = &input.ident;

    let ffi_name = ffi_struct_name(ty_name);
    let free_fn_name = ffi_free_fn_name(ty_name);

    super::FFITypeResolver::insert(&ty_name.to_string(), &ffi_name.to_string());

    quote! {
        #input

        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct #ffi_name {
            inner: *mut core::ffi::c_void,
        }

        impl<T> ezffi::GenericIntoFfi<T> for #ty_name<T> {
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

        impl ezffi::GenericIntoRust for #ffi_name {
            unsafe fn into_rust<T>(&self) -> &T {
                unsafe { &*(self.inner as *mut T) }
            }

            unsafe fn into_rust_mut<T>(&mut self) -> &mut T {
                unsafe { &mut *(self.inner as *mut T) }
            }

            unsafe fn into_rust_owned<T>(self) -> T {
                unsafe { *Box::from_raw(self.inner as *mut T) }
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #free_fn_name(o: #ffi_name) {

            #[allow(clippy::from_raw_with_void_ptr)]
            let _ = unsafe { Box::from_raw(o.inner) };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::normalize;

    use super::*;

    #[test]
    fn struct_macro() {
        let input = quote! {
            pub struct Test {
                number: u64
            }
        };

        let output = expand_simple_struct(input);

        let expected = r#"
            pub struct Test {
                number: u64
            }

            #[derive(Clone, Copy)]
            #[repr(C)]
            pub struct FfiTest {
                inner: *mut core::ffi::c_void,
            }

            impl ezffi::IntoFfi for Test {
                type Ffi = FfiTest;

                unsafe fn ref_into_ffi(&self) -> Self::Ffi {
                    FfiTest {
                        inner: self as *const Self as *mut core::ffi::c_void,
                    }
                }

                unsafe fn owned_into_ffi(self) -> Self::Ffi {
                    FfiTest {
                        inner: Box::into_raw(Box::new(self)) as *mut core::ffi::c_void,
                    }
                }
            }

            impl ezffi::IntoRust for FfiTest {
                type Rust = Test;

                unsafe fn into_rust(&self) -> &Self::Rust {
                    unsafe { &*(self.inner as *mut Self::Rust) }
                }

                unsafe fn into_rust_mut(&mut self) -> &mut Self::Rust {
                    unsafe { &mut *(self.inner as *mut Self::Rust) }
                }

                unsafe fn into_rust_owned(self) -> Self::Rust {
                    unsafe { *Box::from_raw(self.inner as *mut Self::Rust) }
                }
            }

            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn ffi_Test_free(o: FfiTest) {
                let _ = unsafe { ezffi::IntoRust::into_rust_owned(o) };
            }
            "#;

        assert_eq!(normalize(&output.to_string()), normalize(expected));
    }
}
