use convert_case::Casing;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Field, Fields, Ident};

#[proc_macro_derive(Envload)]
pub fn derive_envload(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let fields: Vec<syn::Field> = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields.named.iter().map(|f| f.clone()).collect(),
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    };

    let mut mandatory_fields = vec![];
    // let mut optional_fields = vec![];

    for f in fields {
        // TODO: Check whether the type is mandatory or optional!

        mandatory_fields.push(f);
    }

    let mandatory_tt = gen_mandatory_tt(&mandatory_fields);
    // let optional_tt = gen_optional_tt(&optional_fields);
    let return_struct = gen_return_struct(&name, &mandatory_fields /*, optional_fields */);

    let expanded = quote! {
        impl envload::Envload for #name {
            fn load() -> #name {
                #mandatory_tt

                #return_struct
            }

            // fn try_load() -> Result<#name, envload::EnvloadError> {
            //     unimplemented!()
            // }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn gen_mandatory_tt(fields: &[Field]) -> TokenStream {
    let decls = fields.iter().map(|f| {
        let name = f.ident.clone();
        let key = f
            .ident
            .clone()
            .expect("No field name")
            .clone()
            .to_string()
            .to_case(convert_case::Case::ScreamingSnake);
        let ty = f.ty.clone();

        quote_spanned! {f.span()=>
            let #name: #ty = std::env::var(#key)
                .unwrap_or_else(|_| panic!("Environment variable not found"))
                .parse()
                .unwrap_or_else(|_| panic!("Couldn't parse environment variable"));
        }
    });

    quote! {
        #(#decls)*
    }
}

fn gen_return_struct(name: &Ident, mandatory_fields: &[Field]) -> TokenStream {
    let field_names = mandatory_fields.iter().map(|f| {
        let name = f.ident.clone();
        quote_spanned! {f.span()=>
            #name,
        }
    });

    quote! {
        return #name {
            #(#field_names)*
        };
    }
}

// fn is_optional(field: &Field) -> bool {
//     match field.ty {
//         _ => unimplemented!(),
//     }

//     return false;
// }

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
