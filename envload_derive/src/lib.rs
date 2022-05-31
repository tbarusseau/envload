#![allow(unused)]

use convert_case::Casing;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields};

#[proc_macro_derive(Envload)]
pub fn derive_envload(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl envload::Envload for #name {
            fn load() -> #name {
                unimplemented!()
            }

            // fn try_load() -> Result<#name, envload::EnvloadError> {
            //     unimplemented!()
            // }
        }
    };

    TokenStream::from(expanded)
}

// fn f(data: &Data) -> TokenStream {
//     match *data {
//         Data::Struct(ref data) => match data.fields {
//             Fields::Named(ref fields) => {
//                 let recurse = fields.named.iter().map(|f| {
//                     let name = &f.ident.expect("No field identifier").to_string();
//                     let screaming_snake_case_name =
//                         name.to_case(convert_case::Case::ScreamingSnake);

//                     quote_spanned! {f.span()=>

//                     }
//                 });

//                 quote! {}
//             }
//             Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
//         },
//         Data::Enum(_) | Data::Union(_) => unimplemented!(),
//     }
// }
