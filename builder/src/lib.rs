use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{self, parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let origin_name = input.ident;

    let builder_type = format_ident!("{}Builder", origin_name);
    let data = input.data;
    let origin_fields = builder_struct_fields(&data);

    let builder_fields_def = match origin_fields {
        syn::Fields::Named(ref fields) => {
            let recurse = fields.named.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                quote! {#name: Option<#ty>}
            });
            quote! { #(#recurse),*}
        }
        _ => {
            quote! {}
        }
    };

    let builder_fields_empty = match origin_fields {
        syn::Fields::Named(ref fields) => {
            let recurse = fields.named.iter().map(|f| {
                let field_name = &f.ident;
                quote! {#field_name: None}
            });
            quote! { #(#recurse),*}
        }
        _ => {
            quote! {}
        }
    };

    let tokens = proc_macro::TokenStream::from(quote! {
        pub struct #builder_type {#builder_fields_def}
        impl #origin_name {
            pub fn builder() -> #builder_type {
                #builder_type {#builder_fields_empty}
            }
        }
    });
    eprintln!("TOKENS: {}", tokens);
    tokens
}

fn builder_struct_fields(data: &syn::Data) -> syn::Fields {
    if let syn::Data::Struct(ref ds) = *data {
        ds.fields.clone()
    } else {
        syn::Fields::Unit
    }
    // match *data {
    //     syn::Data::Struct(ref data) => match data.fields {
    //         syn::Fields::Named(ref fields) => {
    //             let recurse = fields.named.iter().map(|f| {
    //                 let name = &f.ident;
    //                 let ty = &f.ty;
    //                 quote!{#name: Option<#ty>}
    //             });
    //             quote!{{ #(#recurse),*}}
    //         }
    //         _ => {
    //             quote! {}
    //         }
    //     },
    //     _ => {
    //         quote! {}
    //     }
    // }
}
