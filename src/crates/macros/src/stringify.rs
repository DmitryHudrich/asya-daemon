use crate::propagate_err;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Ident, Token, Variant, Visibility,
};

pub(crate) fn make_derive(item: TokenStream) -> TokenStream {
    let enum_parser: EnumParser = propagate_err!(syn::parse(item));
    let EnumParser {
        enum_visibility,
        enum_name,
        stringified,
    } = enum_parser;

    quote! {
        impl #enum_name {
            #enum_visibility fn stringify() -> &'static str {
                #stringified
            }
        }
    }
    .into()
}

struct EnumParser {
    enum_visibility: Visibility,
    enum_name: Ident,
    stringified: String,
}

impl Parse for EnumParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _attributes = input.call(Attribute::parse_outer)?;
        let enum_visibility: Visibility = input.parse()?;
        let _enum_token: Token![enum] = input.parse()?;

        let mut data = "enum ".to_string();
        let enum_name: Ident = input.parse()?;
        data.push_str(&enum_name.to_string());
        data.push_str(" {");

        let body;
        let _braces = braced!(body in input);

        let variants: Punctuated<Variant, Token![,]> = body.call(Punctuated::parse_terminated)?;
        let last_variant_index = variants.len() - 1;
        for (i, variant) in variants.into_iter().enumerate() {
            data.push_str(&variant.ident.to_string());

            match variant.fields {
                syn::Fields::Named(fields_named) => {
                    data.push_str(" {");
                    let last = fields_named.named.len() - 1;
                    fields_named
                        .named
                        .iter()
                        .enumerate()
                        .for_each(|(i, field)| {
                            data.push_str(&field.ident.as_ref().unwrap().to_string());
                            data.push_str(": ");
                            data.push_str(&field.ty.to_token_stream().to_string());

                            if i < last {
                                data.push(',');
                            }
                        });
                    data.push('}')
                }
                syn::Fields::Unnamed(fields_unnamed) => {
                    data.push('(');
                    let last = fields_unnamed.unnamed.len() - 1;
                    fields_unnamed
                        .unnamed
                        .iter()
                        .enumerate()
                        .for_each(|(i, field)| {
                            data.push_str(&field.ty.to_token_stream().to_string());
                            if i < last {
                                data.push_str(", ");
                            }
                        });
                    data.push(')');
                }
                syn::Fields::Unit => (),
            }

            if i < last_variant_index {
                data.push(',');
            }
        }

        data.push('}');

        let syntax_tree = syn::parse_file(&data).unwrap();
        data = prettyplease::unparse(&syntax_tree);

        Ok(EnumParser {
            enum_visibility,
            enum_name,
            stringified: data,
        })
    }
}
