use std::{env, sync::LazyLock};

use quote::quote;
use syn::{Item, parse_macro_input};

use crate::{
    functions::{expand_fn, expand_impl},
    structs::{expand_c_enum, expand_enum, expand_struct},
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
    let output: proc_macro2::TokenStream =
        export_impl(attr, item, GenerationType::External, false).into();

    quote! {
        #output
    }
    .into()
}

#[proc_macro]
pub fn export_extern_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = parse_macro_input!(input as syn::Path);

    let dummy_struct = quote! { struct #path {} }.into();

    export_impl(
        proc_macro::TokenStream::new(),
        dummy_struct,
        GenerationType::Internal,
        true,
    )
}

fn export_impl(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
    generation_type: GenerationType,
    mut skip_input: bool,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as Item);

    let output: proc_macro2::TokenStream = match &item {
        Item::Struct(item) => expand_struct(item, generation_type),
        Item::Enum(item) => {
            if item
                .variants
                .iter()
                .all(|v| matches!(v.fields, syn::Fields::Unit))
            {
                skip_input = true;
                expand_c_enum(item, generation_type)
            } else {
                expand_enum(item, generation_type)
            }
        }
        Item::Fn(item) => expand_fn(item),
        Item::Impl(item) => expand_impl(item),
        _ => unimplemented!("#[ezffi::export] not supported item {}", quote! { #item }),
    };

    if skip_input {
        quote! {
            #output
        }
        .into()
    } else {
        quote! {
            #item
            #output
        }
        .into()
    }
}
