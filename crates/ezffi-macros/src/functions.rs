use quote::quote;
use syn::{FnArg, ItemFn, ItemImpl, ReturnType, Signature, Type};

use crate::{FFINamer, FFITypeResolver};

pub fn expand_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let input: ItemImpl = syn::parse2(input).expect("Must be valid code");
    let impl_ty = &input.self_ty;

    let mut wrappers = Vec::new();

    for impl_item in &input.items {
        if let syn::ImplItem::Fn(method) = impl_item {
            wrappers.push(generate_fn_wrapper(Some(impl_ty), &method.sig));
        }
    }

    quote! { #( #wrappers )* }
}

pub fn expand_fn(item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let input: ItemFn = syn::parse2(item).expect("Must be valid code");

    let wrapper = generate_fn_wrapper(None, &input.sig);

    quote! { #wrapper }
}

fn generate_fn_wrapper(impl_ty: Option<&Type>, sig: &Signature) -> proc_macro2::TokenStream {
    let fn_name = &sig.ident;
    let inputs = sig.inputs.iter().collect::<Vec<_>>();
    let output = &sig.output;

    let mut ffi_params = Vec::new();
    let mut conversions = Vec::new();
    let mut call_args = Vec::new();

    // Generate the FFI function name
    let ffi_fn_name = FFINamer::name_fn(fn_name, impl_ty);

    for arg in inputs {
        match arg {
            FnArg::Receiver(receiver) => {
                let ffi_ty = super::FFITypeResolver::ffi_ty_for(&receiver.ty, impl_ty);

                let is_ref = receiver.reference.is_some();
                let is_mut = receiver.mutability.is_some();

                let self_conversion = match (is_ref, is_mut) {
                    (false, false) => quote! {
                        let mut this = &mut *(this as *mut #ffi_ty);
                        let mut this = this.into_rust_owned();
                    },
                    (true, false) => quote! {
                        let mut this = &*this;
                        let this = this.into_rust();
                    },
                    (false, true) => quote! {
                        let this = &*this;
                        let mut this = this.into_rust_owned();
                    },
                    (true, true) => quote! {
                        let mut this = *this;
                        let mut this = this.into_rust_mut();
                    },
                };

                ffi_params.push(quote! { mut this: *const #ffi_ty });
                conversions.push(self_conversion);
                call_args.push(quote! { this });
            }
            FnArg::Typed(pat_type) => {
                let name = match &*pat_type.pat {
                    syn::Pat::Ident(ident) => &ident.ident,
                    _ => {
                        unimplemented!("Unsupported parameter pattern {}", quote!(*pat_type.pat))
                    }
                };
                let ty = &pat_type.ty;

                if FFITypeResolver::is_primitive(ty) {
                    ffi_params.push(quote! { mut #name: #ty });
                } else {
                    let ffi_ty = super::FFITypeResolver::ffi_ty_for(ty, impl_ty);

                    let ty_conversion = match &*pat_type.ty {
                        Type::Reference(r) => {
                            if r.mutability.is_some() {
                                quote! {
                                    let mut #name = &mut *(#name as *mut #ffi_ty);
                                    let mut #name = #name.into_rust_mut();
                                }
                            } else {
                                quote! {
                                    let #name = &*#name;
                                    let #name = #name.into_rust();
                                }
                            }
                        }
                        Type::Path(_) => {
                            quote! {
                                let mut #name = *#name;
                                let mut #name = #name.into_rust_owned();
                            }
                        }
                        _ => unimplemented!("Unsupported parameter with type {}", quote! { #ty }),
                    };

                    ffi_params.push(quote! { mut #name: *const #ffi_ty });
                    conversions.push(ty_conversion);
                }

                call_args.push(quote! { #name });
            }
        }
    }

    // Call the function using full qualified name, have to check
    // if it is a method or a free function
    let call = if let Some(ty) = impl_ty {
        match ty {
            syn::Type::Path(path) => {
                let ident = &path.path.segments[0].ident;
                quote! { #ident::#fn_name( #( #call_args ),* ) }
            }
            _ => unimplemented!("Cannot call method on type {}", quote! { #ty }),
        }
    } else {
        quote! { #fn_name( #( #call_args ),* ) }
    };

    // Function return type
    let ffi_ret = match output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => {
            if FFITypeResolver::is_primitive(ty) {
                let ty = super::FFITypeResolver::ffi_ty_for(ty, impl_ty);
                quote! { #ty }
            } else {
                let ty = super::FFITypeResolver::ffi_ty_for(ty, impl_ty);
                quote! { *const #ty }
            }
        }
    };

    // Return conversion
    let ret_conversion = match output {
        ReturnType::Default => quote! {},
        ReturnType::Type(_, ty) => {
            if FFITypeResolver::is_primitive(ty) {
                quote! { result }
            } else {
                match &**ty {
                    syn::Type::Reference(_) => quote! { result.ref_into_ffi() },
                    syn::Type::Path(_) => quote! { result.owned_into_ffi() },
                    _ => unimplemented!("Return type unsupported {}", quote! { #ty }),
                }
            }
        }
    };

    quote! {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #ffi_fn_name(
            #( #ffi_params ),*
        ) -> #ffi_ret {
            use ezffi::IntoFfi;
            use ezffi::IntoRust;

            #( #conversions )*

            let result = #call;

            #ret_conversion
        }
    }
}
