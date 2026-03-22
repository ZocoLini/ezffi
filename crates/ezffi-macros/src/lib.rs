use std::{env, sync::LazyLock};

use quote::quote;
use syn::{Item, parse_macro_input};

use crate::{
    functions::{expand_fn, expand_impl},
    structs::expand_struct,
};

mod config;
mod functions;
mod namer;
mod structs;
mod type_resolver;

use config::CONFIG;
use namer::FFINamer;
use type_resolver::FFITypeResolver;

static PKG_NAME: LazyLock<String> = LazyLock::new(|| env::var("CARGO_PKG_NAME").unwrap());
static MANIFEST_DIR: LazyLock<String> = LazyLock::new(|| env::var("CARGO_MANIFEST_DIR").unwrap());

enum GenerationType {
    Internal,
    External,
}

#[proc_macro_attribute]
pub fn export(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = {
        let item = item.clone();
        parse_macro_input!(item as Item)
    };

    let output: proc_macro2::TokenStream = export_impl(attr, item, GenerationType::External).into();

    quote! {
        #input
        #output
    }
    .into()
}

fn export_impl(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
    generation_type: GenerationType,
) -> proc_macro::TokenStream {
    let item_to_export = {
        let item = item.clone();
        parse_macro_input!(item as Item)
    };

    match item_to_export {
        Item::Struct(item) => expand_struct(item, generation_type).into(),
        Item::Fn(_) => expand_fn(item.into()).into(),
        Item::Impl(_) => expand_impl(item.into()).into(),
        _ => unimplemented!(
            "#[ezffi::export] not supported item {}",
            quote! { #item_to_export }
        ),
    }
}

#[proc_macro]
pub fn export_extern_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = parse_macro_input!(input as syn::Path);

    let dummy_struct = quote! { struct #path {} }.into();

    export_impl(
        proc_macro::TokenStream::new(),
        dummy_struct,
        GenerationType::Internal,
    )
}
