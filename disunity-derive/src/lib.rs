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
    todo!()
}
