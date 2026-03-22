use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, RwLock},
};

use quote::{format_ident, quote};
use syn::Type;

// Maps Rust type names to FFI type names
static TYPE_NAME_MAP: LazyLock<Arc<RwLock<HashMap<String, String>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

const PRIMITIVE_TYPES: &[&str] = &[
    "bool", "char", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "i128", "u128", "isize",
    "usize", "f32", "f64",
];

pub struct FFITypeResolver;

impl FFITypeResolver {
    pub fn insert(ty_name: &str, ffi_ty_name: &str) {
        TYPE_NAME_MAP
            .write()
            .unwrap()
            .insert(ty_name.to_string(), ffi_ty_name.to_string());
    }

    pub fn is_primitive(ty: &syn::Type) -> bool {
        if let syn::Type::Path(p) = ty {
            if let Some(ident) = p.path.get_ident() {
                PRIMITIVE_TYPES.contains(&ident.to_string().as_str())
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn ffi_ty_for(ty: &syn::Type, self_repl: Option<&Type>) -> proc_macro2::TokenStream {
        if Self::is_primitive(ty) {
            return quote! { #ty };
        }

        match ty {
            syn::Type::Reference(r) => {
                let ty = Self::ffi_ty_for(&r.elem, self_repl);

                quote! { #ty }
            }
            syn::Type::Path(path) => {
                if path.path.segments[0].ident == "Self" {
                    return Self::ffi_ty_for(self_repl.unwrap(), None);
                }

                let ident = path.path.segments[0].ident.to_string();
                let ty_name = ident.to_string();

                let map_lock = TYPE_NAME_MAP.read().unwrap();

                if let Some(ffi_ty) = map_lock.get(&ty_name) {
                    let ty = format_ident!("{}", ffi_ty);
                    quote! { #ty }
                } else {
                    let ty = format_ident!("EzFfi{}", ident);
                    quote! { ezffi::#ty }
                }
            }
            _ => unimplemented!("Unsupported ty {}", quote!(#ty)),
        }
    }
}
