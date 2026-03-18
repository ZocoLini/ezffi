use syn::DeriveInput;

use crate::{ffi_free_fn_name, ffi_struct_name};
use quote::quote;

pub fn expand_struct(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
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

        let output = expand_struct(input);

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
