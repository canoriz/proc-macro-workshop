use quote::{format_ident, quote};
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

    let builder_setter_funcs = match origin_fields {
        syn::Fields::Named(ref fields) => {
            let recurse = fields.named.iter().map(|f| {
                let field_name = &f.ident;
                let field_type = &f.ty;
                quote! {
                    fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                        self.#field_name = Some(#field_name);
                        self
                    }
                }
            });
            quote! { #(#recurse)*}
        }
        _ => {
            quote! {}
        }
    };

    let check_builder_field_not_none = match origin_fields {
        syn::Fields::Named(ref fields) => {
            let recurse = fields.named.iter().map(|f| {
                let field_name = &f.ident;
                let field_name_string = quote!{
                    #field_name
                }.to_string();
                quote! {
                    if let None = self.#field_name {
                        return Err(format!("{} not set", #field_name_string).into());
                    }
                }
            });
            quote! { #(#recurse)*}
        }
        _ => {
            quote! {}
        }
    };

    let builder_fields_filled = match origin_fields {
        syn::Fields::Named(ref fields) => {
            let recurse = fields.named.iter().map(|f| {
                let field_name = &f.ident;
                quote! {#field_name: self.#field_name.clone().unwrap()}
            });
            quote! { #(#recurse),*}
        }
        _ => {
            quote! {}
        }
    };

    let tokens = proc_macro::TokenStream::from(quote! {
        struct #builder_type {#builder_fields_def}
        impl #origin_name {
            pub fn builder() -> #builder_type {
                #builder_type {#builder_fields_empty}
            }
        }
        impl #builder_type {
            #builder_setter_funcs
            fn build(&mut self) -> Result<#origin_name, Box<dyn std::error::Error>> {
                #check_builder_field_not_none
                Ok(#origin_name{
                    #builder_fields_filled
                })
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
}
