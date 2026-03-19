use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, RwLock},
};

use quote::{format_ident, quote};
use syn::Type;

// Maps Rust type names to FFI type names
static TYPE_NAME_MAP: LazyLock<Arc<RwLock<HashMap<String, String>>>> = LazyLock::new(|| {
    let mut hashmap = HashMap::new();

    // primitives compatible with FFI by default
    hashmap.insert("i8".to_string(), "i8".to_string());
    hashmap.insert("u8".to_string(), "u8".to_string());
    hashmap.insert("i16".to_string(), "i16".to_string());
    hashmap.insert("u16".to_string(), "u16".to_string());
    hashmap.insert("i32".to_string(), "i32".to_string());
    hashmap.insert("u32".to_string(), "u32".to_string());
    hashmap.insert("i64".to_string(), "i64".to_string());
    hashmap.insert("u64".to_string(), "u64".to_string());
    hashmap.insert("i128".to_string(), "i128".to_string());
    hashmap.insert("u128".to_string(), "u128".to_string());
    hashmap.insert("isize".to_string(), "isize".to_string());
    hashmap.insert("usize".to_string(), "usize".to_string());

    hashmap.insert("f32".to_string(), "f32".to_string());
    hashmap.insert("f64".to_string(), "f64".to_string());

    hashmap.insert("bool".to_string(), "bool".to_string());
    hashmap.insert("char".to_string(), "char".to_string());

    Arc::new(RwLock::new(hashmap))
});

pub struct FFITypeResolver;

impl FFITypeResolver {
    pub fn insert(ty_name: &str, ffi_ty_name: &str) {
        TYPE_NAME_MAP
            .write()
            .unwrap()
            .insert(ty_name.to_string(), ffi_ty_name.to_string());
    }

    pub fn ffi_ty_for(ty: &syn::Type, self_repl: Option<&Type>) -> proc_macro2::TokenStream {
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
                    let ty = format_ident!("{}", ident);
                    quote! { ezffi::#ty }
                }
            }
            _ => unimplemented!("Unsupported ty {}", quote!(#ty)),
        }
    }
}
