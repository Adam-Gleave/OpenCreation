use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{
    parenthesized, parse2, parse_macro_input, Attribute, Data, DeriveInput, Fields, LitBool, Token, TypeParam,
};

#[proc_macro_derive(Form)]
pub fn derive_form(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let type_params = generics.type_params().collect::<Vec<&TypeParam>>();

    let form_impl = if type_params.len() > 0 {
        generate_impl(name, &Ident::new("Form", Span::call_site()), type_params)
    } else {
        quote! { impl Form for #name }
    };

    let expanded = quote! {
        #form_impl {
            fn form_id(&self) -> u32 {
                self.header.id
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(Readable, attributes(record_header, subrecord_header, size_var))]
pub fn derive_readable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let type_params = generics.type_params().collect::<Vec<&TypeParam>>();

    let progress_reader = generate_progress_reader(input.attrs);

    let read_fn_impl = if type_params.len() > 0 {
        generate_impl(name, &Ident::new("Readable", Span::call_site()), type_params)
    } else {
        quote! { impl Readable for #name }
    };

    let read_fn_body = generate_read_fn(&input.data, progress_reader);

    let expanded = quote! {
        #read_fn_impl {
            fn read(reader: &mut EspReader) -> ::std::io::Result<Self> {
                #read_fn_body
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn generate_impl(name: &Ident, trait_name: &Ident, type_params: Vec<&TypeParam>) -> TokenStream {
    let recurse = type_params.iter().map(|g| {
        quote! { #g }
    });

    let types_recurse = recurse.clone();
    let where_recurse = recurse.clone();
    let types_literal = quote! { <#(#types_recurse, )*> };
    let where_literal = quote! { #(#where_recurse: Readable,)* };

    quote! {
        impl#types_literal #trait_name for #name#types_literal
        where
            #where_literal
    }
}

fn generate_read_fn(data: &Data, progress_reader: ProgressReaderGenerated) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse_assign = fields.named.iter().map(|f| {
                    let t = &f.ty;
                    let name = &f.ident;
                    let name_str = &f.ident.as_ref().unwrap().to_string();
                    let progress_reader_fn = &progress_reader.tokens;

                    if progress_reader.params.is_some()
                        && name_str == &progress_reader.params.as_ref().unwrap().size_struct.to_string()
                    {
                        quote! {
                            let #name = <#t>::read(reader)?;
                            #progress_reader_fn
                        }
                    } else {
                        quote! {
                            let #name = <#t>::read(reader)?;
                        }
                    }
                });
                let recurse_fields = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        #name,
                    }
                });
                quote! {
                    #(#recurse_assign)*
                    Ok(Self {
                        #(#recurse_fields)*
                    })
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn generate_progress_reader(attributes: Vec<Attribute>) -> ProgressReaderGenerated {
    let record_header_attr = attributes
        .iter()
        .find(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "record_header");
    let subrecord_header_attr = attributes
        .iter()
        .find(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "subrecord_header");
    let size_var_attr = attributes.iter().find(|a| a.path.segments[0].ident == "size_var");

    let mut token_stream = TokenStream::new();

    let size_var_params = if record_header_attr.is_some() && subrecord_header_attr.is_some() {
        panic!("Struct cannot be both a record header and a subrecord header");
    } else if record_header_attr.is_some() && size_var_attr.is_some() {
        let record_header_params: ReadableRecordParams =
            parse2(record_header_attr.unwrap().tokens.clone()).expect("Error parsing \"record_header\" attribute");
        let size_var_params: ReadableSizeVarParams =
            parse2(size_var_attr.unwrap().tokens.clone()).expect("Error parsing \"size_var\" attribute");

        if !record_header_params.record_header.value {
            panic!("Expected boolean \"true\" on header type to enable EspReader progression");
        }

        let size_struct = &size_var_params.size_struct;
        let size_var = &size_var_params.size_var;

        token_stream = quote! {
            reader.next_record_data(#size_struct.#size_var);
        };
        Some(size_var_params)
    } else if subrecord_header_attr.is_some() && size_var_attr.is_some() {
        let subrecord_header_params: ReadableSubrecordParams = parse2(subrecord_header_attr.unwrap().tokens.clone())
            .expect("Error parsing \"subrecord_header\" attribute");
        let size_var_params: ReadableSizeVarParams =
            parse2(size_var_attr.unwrap().tokens.clone()).expect("Error parsing \"size_var\" attribute");

        if !subrecord_header_params.subrecord_header.value {
            panic!("Expected boolean \"true\" on header type to enable EspReader progression");
        }

        let size_struct = &size_var_params.size_struct;
        let size_var = &size_var_params.size_var;

        token_stream = quote! {
            reader.next_subrecord_data(#size_struct.#size_var);
        };
        Some(size_var_params)
    } else {
        None
    };

    ProgressReaderGenerated {
        tokens: token_stream,
        params: size_var_params,
    }
}

struct ReadableRecordParams {
    record_header: LitBool,
}

struct ReadableSubrecordParams {
    subrecord_header: LitBool,
}

struct ReadableSizeVarParams {
    size_var: Ident,
    size_struct: Ident,
}

impl Parse for ReadableRecordParams {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let record_header = content.parse()?;

        Ok(ReadableRecordParams { record_header })
    }
}

impl Parse for ReadableSubrecordParams {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let subrecord_header = content.parse()?;

        Ok(ReadableSubrecordParams { subrecord_header })
    }
}

impl Parse for ReadableSizeVarParams {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let size_struct = content.parse()?;
        content.parse::<Token![,]>()?;
        let size_var = content.parse()?;

        Ok(ReadableSizeVarParams { size_struct, size_var })
    }
}

struct ProgressReaderGenerated {
    tokens: TokenStream,
    params: Option<ReadableSizeVarParams>,
}
