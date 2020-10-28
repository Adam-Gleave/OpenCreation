use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Readable)]
pub fn derive_readable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let read_fn = generate_read_fn(&input.data);

    let expanded = quote! {
        impl Readable for #name {
            fn read(reader: &mut EspReader) -> ::std::io::Result<Self> {
                Ok(Self {
                    #read_fn
                })
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn generate_read_fn(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let t = &f.ty;
                    let name = &f.ident;
                    quote! {
                        #name: #t::read(reader)?,
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
