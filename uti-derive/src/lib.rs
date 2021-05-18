use std::convert::TryInto;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Fields};

#[proc_macro_derive(UniversalType)]
pub fn derive_universal_type(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::DeriveInput);

    let struct_name = item.ident;
    let mut type_layout = String::new();
    type_layout.push_str(&struct_name.to_string());

    // TODO: Generics.
    type_layout.push_str("<>");

    match item.data {
        syn::Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let children_string = fields
                    .named
                    .iter()
                    .map(|field| field.ty.to_token_stream().to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                type_layout.push_str("{");
                type_layout.push_str(&children_string);
                type_layout.push_str("}");
            }
            Fields::Unnamed(ref fields) => {
                let children_string = fields
                    .unnamed
                    .iter()
                    .map(|field| field.ty.to_token_stream().to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                type_layout.push_str("(");
                type_layout.push_str(&children_string);
                type_layout.push_str(")");
            }
            Fields::Unit => {
                type_layout.push_str(".");
            }
        },
        syn::Data::Enum(_) => {
            unimplemented!()
        }
        syn::Data::Union(_) => {
            unimplemented!()
        }
    }
    let hash = compute_hash(&type_layout);

    proc_macro::TokenStream::from(quote! {
        impl uti::UniversalType for #struct_name {
            const UNIVERSAL_TYPE_ID_BYTES: [u8;uti::MAX_UTI_BYTES] = [#(#hash),*];
        }
    })
}

fn compute_hash(s: &str) -> [u8; uti::MAX_UTI_BYTES] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(s.as_bytes());

    let result = hasher.finalize();
    result.as_bytes()[..uti::MAX_UTI_BYTES].try_into().unwrap()
}
