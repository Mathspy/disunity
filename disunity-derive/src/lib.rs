use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, Data, DataStruct, DataUnion, DeriveInput};

#[proc_macro_derive(Variants)]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    TokenStream::from(inner(input))
}

fn inner(input: DeriveInput) -> TokenStream2 {
    let _data = match input.data {
        Data::Struct(DataStruct { struct_token, .. }) => {
            return syn::Error::new(struct_token.span, "Variants can't be derived on structs")
                .into_compile_error();
        }
        Data::Union(DataUnion { union_token, .. }) => {
            return syn::Error::new(union_token.span, "Variants can't be derived on unions")
                .into_compile_error();
        }
        Data::Enum(data) => data,
    };

    todo!()
}
