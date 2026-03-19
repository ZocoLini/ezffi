use quote::quote;
use syn::{FnArg, ItemFn, ItemImpl, ReturnType, Signature, Type};

use crate::ffi_fn_name;

pub fn expand_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let input: ItemImpl = syn::parse2(input).expect("Must be valid code");
    let impl_ty = &input.self_ty;

    let mut wrappers = Vec::new();

    for impl_item in &input.items {
        if let syn::ImplItem::Fn(method) = impl_item {
            wrappers.push(generate_fn_wrapper(Some(impl_ty), &method.sig));
        }
    }

    quote! {
        #input
        #( #wrappers )*
    }
}

pub fn expand_fn(item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let input: ItemFn = syn::parse2(item).expect("Must be valid code");

    let wrapper = generate_fn_wrapper(None, &input.sig);

    quote! {
        #input
        #wrapper
    }
}

fn generate_fn_wrapper(impl_ty: Option<&Type>, sig: &Signature) -> proc_macro2::TokenStream {
    let fn_name = &sig.ident;
    let inputs = sig.inputs.iter().collect::<Vec<_>>();
    let output = &sig.output;

    let mut ffi_params = Vec::new();
    let mut conversions = Vec::new();
    let mut call_args = Vec::new();

    // Generate the FFI function name
    let ffi_fn_name = ffi_fn_name(fn_name, impl_ty);

    for arg in inputs {
        match arg {
            FnArg::Receiver(receiver) => {
                let ffi_ty = super::FFITypeResolver::ffi_ty_for(&receiver.ty, impl_ty);

                let is_ref = receiver.reference.is_some();
                let is_mut = receiver.mutability.is_some();

                let self_conversion = match (is_ref, is_mut) {
                    (false, false) => quote! {
                        let mut this = this.into_rust_owned();
                    },
                    (true, false) => quote! {
                        let this = this.into_rust();
                    },
                    (false, true) => quote! {
                        let mut this = this.into_rust_owned();
                    },
                    (true, true) => quote! {
                        let mut this = this.into_rust_mut();
                    },
                };

                ffi_params.push(quote! { mut this: #ffi_ty });
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
                let ffi_ty = super::FFITypeResolver::ffi_ty_for(ty, impl_ty);

                let ty_conversion = match &*pat_type.ty {
                    Type::Reference(r) => {
                        if r.mutability.is_some() {
                            quote! {
                                let mut #name = #name.into_rust_mut();
                            }
                        } else {
                            quote! {
                                let #name = #name.into_rust();
                            }
                        }
                    }
                    Type::Path(_) => {
                        quote! {
                            let mut #name = #name.into_rust_owned();
                        }
                    }
                    _ => unimplemented!("Unsupported parameter with type {}", quote! { #ty }),
                };

                ffi_params.push(quote! { mut #name: #ffi_ty });
                conversions.push(ty_conversion);
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
        ReturnType::Type(_, ty) => super::FFITypeResolver::ffi_ty_for(ty, impl_ty),
    };

    // Return conversion
    let ret_conversion = match output {
        ReturnType::Default => quote! { result.owned_into_ffi() },
        ReturnType::Type(_, ty) => match &**ty {
            syn::Type::Reference(_) => quote! { result.ref_into_ffi() },
            syn::Type::Path(_) => quote! { result.owned_into_ffi() },
            _ => unimplemented!("Return type unsupported {}", quote! { #ty }),
        },
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
