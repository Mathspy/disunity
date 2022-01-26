use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, punctuated::Punctuated, Attribute, Data, DataStruct, DataUnion, DeriveInput,
    Expr, ExprLit, Fields, Meta, NestedMeta, Token,
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
    let original_name = input.ident;
    let variants = data
        .variants
        .into_iter()
        .filter(|variant| variant.ident != "Unknown")
        .map(|mut variant| {
            let attrs = std::mem::take(&mut variant.attrs);

            let meta = get_disunity_attr(attrs)
                .map_err(|error| match error {
                    GetAttrError::ExtraAttribute(attribute) => syn::Error::new_spanned(
                        attribute,
                        "unexpected second #[disunity] attribute macro on variant",
                    ),
                    GetAttrError::MissingAttribute => syn::Error::new_spanned(
                        &variant,
                        "variant without a #[disunity(discriminant = N)] attribute",
                    ),
                })
                .and_then(|attribute| attribute.parse_meta())?;

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

            let mut original_fields = Fields::Unit;
            std::mem::swap(&mut variant.fields, &mut original_fields);
            variant.fields = Fields::Unit;
            variant.discriminant = Some((
                discriminant.eq_token,
                Expr::from(ExprLit {
                    lit: discriminant.lit.clone(),
                    attrs: Vec::new(),
                }),
            ));

            let literal = discriminant.lit;
            let ident = variant.ident.clone();

            let from_int_arm = quote! { #literal => Some(Self::#ident) };

            // We want to create the enum from the variants only if the original variant
            // has no fields
            let from_variant_arm = if let Fields::Unit = original_fields {
                Some(quote! { #name::#ident => Some(Self::#ident) })
            } else {
                None
            };
            Ok((variant, from_int_arm, from_variant_arm))
        })
        .try_fold::<_, _, Result<(_, _, _), syn::Error>>(
            (
                <Punctuated<_, Token![,]>>::new(),
                <Punctuated<_, Token![,]>>::new(),
                <Punctuated<_, Token![,]>>::new(),
            ),
            |(mut variants, mut from_int_arms, mut from_variant_arms), result| {
                let (variant, from_int_arm, from_variant_arm) = result?;
                variants.push(variant);
                from_int_arms.push(from_int_arm);
                if let Some(from_variant_arm) = from_variant_arm {
                    from_variant_arms.push(from_variant_arm);
                }

                Ok((variants, from_int_arms, from_variant_arms))
            },
        );

    let (variants, from_int_arms, from_variant_arms) = match variants {
        Ok((variants, from_int_arms, from_variant_arms)) => {
            (variants, from_int_arms, from_variant_arms)
        }
        Err(error) => return error.into_compile_error(),
    };

    // There may not be any variant conversions if all variants have fields
    let from_variants = if from_variant_arms.is_empty() {
        quote! {}
    } else {
        quote! {
            impl #original_name {
                fn from_variant(variant: #name) -> Option<Self> {
                    match variant {
                       #from_variant_arms,
                       _ => None,
                    }
                }
            }
        }
    };

    quote! {
        #[derive(Debug, PartialEq)]
        enum #name {
            #variants
        }

        impl #name {
            fn from_int(value: isize) -> Option<Self> {
                match value {
                   #from_int_arms,
                   _ => None,
                }
            }
        }

        #from_variants
    }
    .into_token_stream()
}

#[repr(u8)]
enum GetAttrError {
    ExtraAttribute(Attribute),
    MissingAttribute,
}

fn get_disunity_attr(attrs: Vec<Attribute>) -> Result<Attribute, GetAttrError> {
    let mut attributes = attrs
        .into_iter()
        .filter(|attribute| {
            attribute
                .path
                .get_ident()
                .map(|ident| ident == "disunity")
                .unwrap_or(false)
        })
        .map(|attribute| attribute);

    match (attributes.next(), attributes.next()) {
        (Some(attribute), None) => Ok(attribute),
        (Some(_), Some(attribute)) => Err(GetAttrError::ExtraAttribute(attribute)),
        (None, _) => Err(GetAttrError::MissingAttribute),
    }
}
