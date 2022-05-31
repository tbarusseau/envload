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
    let decls = gen_decls(&fields);
    let return_struct = gen_return_struct(&name, &fields);

    let expanded = quote! {
        use envload::maybe_option::__private::{MaybeOption, GenerateFallback};

        impl envload::LoadEnv for #name {
            fn load_env() -> #name {
                #decls

                #return_struct
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

/// Generate the field declarations:
/// ```rust
///     let secret_key: String = /* ... */;
///     let optional_data: Option<T> = /* ... */;
/// ```
fn gen_decls(fields: &[Field]) -> TokenStream {
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
            let #name: #ty = <MaybeOption<#ty>>::new().generate(#key);
        }
    });

    quote! {
        #(#decls)*
    }
}

/// Generate the return struct based on input fields:
/// ```rust
/// return Env {
///     secret_key,
///     optional_data,
/// };
/// ```
fn gen_return_struct(name: &Ident, fields: &[Field]) -> TokenStream {
    let field_names = fields.iter().map(|f| {
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
