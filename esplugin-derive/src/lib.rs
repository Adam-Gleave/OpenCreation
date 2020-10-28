use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{
    parenthesized, parse2, parse_macro_input, Data, DeriveInput, Fields, Ident, Token, Type,
    TypeParam, Variant,
};

#[proc_macro_derive(Readable)]
pub fn derive_readable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let type_params = generics.type_params().collect::<Vec<&TypeParam>>();

    let read_fn_impl = if type_params.len() > 0 {
        generate_impl(name, type_params)
    } else {
        quote! { impl Readable for #name }
    };

    let read_fn_body = generate_read_fn(&input.data);

    let expanded = quote! {
        #read_fn_impl {
            fn read(reader: &mut EspReader) -> ::std::io::Result<Self> {
                Ok(Self {
                    #read_fn_body
                })
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn generate_impl(name: &Ident, type_params: Vec<&TypeParam>) -> TokenStream {
    let recurse = type_params.iter().map(|g| {
        quote! { #g }
    });

    let types_recurse = recurse.clone();
    let where_recurse = recurse.clone();

    let types_literal = quote! { <#(#types_recurse, )*> };
    let where_literals = quote! { #(#where_recurse: Readable,)* };

    quote! {
        impl#types_literal Readable for #name#types_literal
        where
            #where_literals
    }
}

fn generate_read_fn(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let t = &f.ty;
                    let name = &f.ident;
                    quote! {
                        #name: <#t>::read(reader)?,
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

struct CodedParams {
    return_type: Type,
    enum_variant: Variant,
}

impl Parse for CodedParams {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let return_type = content.parse()?;
        content.parse::<Token![,]>()?;
        let enum_variant = content.parse()?;

        Ok(CodedParams {
            return_type,
            enum_variant,
        })
    }
}

#[proc_macro_derive(Coded, attributes(code_type))]
pub fn derive_coded(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let attribute = input
        .attrs
        .iter()
        .filter(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "code_type")
        .nth(0)
        .expect("\"code_type\" attribute expected for deriving Coded trait");

    let params: CodedParams = parse2(attribute.tokens.clone()).expect(&format!(
        "Invalid Coded attribute, expected \"code_type\": Tokens {}",
        attribute.tokens
    ));

    let return_type = params.return_type;
    let enum_variant = params.enum_variant;

    let expanded = quote! {
        impl Coded<#return_type> for #name {
            fn code() -> #return_type {
                #enum_variant
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
