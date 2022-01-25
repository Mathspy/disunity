use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, punctuated::Punctuated, Data, DataStruct, DataUnion, DeriveInput, Expr,
    ExprLit, Fields, Meta, NestedMeta, Token, Variant,
};

#[proc_macro_derive(Variant, attributes(disunity))]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    TokenStream::from(inner(input))
}

fn inner(input: DeriveInput) -> TokenStream2 {
    let data = match input.data {
        Data::Struct(DataStruct { struct_token, .. }) => {
            return syn::Error::new(struct_token.span, "Variant can't be derived on structs")
                .into_compile_error();
        }
        Data::Union(DataUnion { union_token, .. }) => {
            return syn::Error::new(union_token.span, "Variant can't be derived on unions")
                .into_compile_error();
        }
        Data::Enum(data) => data,
    };

    let name = format_ident!("{}Variant", input.ident);
    let variants = data
        .variants
        .into_iter()
        .filter(|variant| variant.ident != "Unknown")
        .map(|mut variant| {
            let attrs = std::mem::take(&mut variant.attrs);

            let mut attributes = attrs
                .into_iter()
                .filter(|attribute| {
                    attribute
                        .path
                        .get_ident()
                        .map(|ident| ident == "disunity")
                        .unwrap_or(false)
                })
                .map(|attribute| (attribute.parse_meta(), attribute));

            let meta = match (attributes.next(), attributes.next()) {
                (Some((Ok(meta), _)), None) => meta,
                (Some(_), Some((_, attribute))) => {
                    return Err(syn::Error::new_spanned(
                        attribute,
                        "unexpected second #[disunity] attribute macro on variant",
                    ));
                }
                (Some((Err(error), _)), _) => return Err(error),
                (None, _) => {
                    return Err(syn::Error::new_spanned(
                        variant,
                        "variant without a #[disunity(discriminant = N)] attribute",
                    ))
                }
            };

            let (mut attribute_meta_list, nested) = match meta {
                Meta::List(list) => {
                    let nested = list.nested.clone();
                    let iter = list
                        .nested
                        .into_iter()
                        .map(|nested_meta| match nested_meta {
                            NestedMeta::Meta(inner_meta) => match inner_meta {
                                Meta::NameValue(name_value)
                                    if name_value
                                        .path
                                        .get_ident()
                                        .map(|ident| ident == "discriminant")
                                        .unwrap_or(false) =>
                                {
                                    Ok(name_value)
                                }
                                _ => Err(syn::Error::new_spanned(
                                    inner_meta,
                                    "expected discriminant = N inside of disunity attribute",
                                )),
                            },
                            NestedMeta::Lit(lit) => Err(syn::Error::new_spanned(
                                lit,
                                "unexpected literal in disunity attribute macro",
                            )),
                        });

                    (iter, nested)
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        meta,
                        "expected #[disunity(discriminant = N)] attribute",
                    ));
                }
            };

            let discriminant = match (attribute_meta_list.next(), attribute_meta_list.next()) {
                (Some(Ok(discriminant)), None) => discriminant,
                (Some(Ok(_)), Some(Ok(_))) => {
                    return Err(syn::Error::new_spanned(
                        nested,
                        "expected only one discriminant = N inside of disunity attribute",
                    ));
                }
                (Some(Err(error)), _) => return Err(error),
                (_, Some(Err(error))) => return Err(error),
                (None, _) => {
                    return Err(syn::Error::new_spanned(
                        nested,
                        "excepted discriminant = N in disunity attribute macro",
                    ))
                }
            };

            variant.fields = Fields::Unit;
            variant.discriminant = Some((
                discriminant.eq_token,
                Expr::from(ExprLit {
                    lit: discriminant.lit,
                    attrs: Vec::new(),
                }),
            ));
            Ok(variant)
        })
        .collect::<Result<Punctuated<Variant, Token![,]>, syn::Error>>();

    let variants = match variants {
        Ok(variants) => variants,
        Err(error) => return error.into_compile_error(),
    };

    quote! {
        enum #name {
            #variants
        }
    }
    .into_token_stream()
}
