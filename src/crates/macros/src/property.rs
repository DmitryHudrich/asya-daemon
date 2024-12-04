use std::collections::HashMap;

use crate::propagate_err;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    braced, ext::IdentExt, parenthesized, parse::Parse, parse_macro_input, punctuated::Punctuated,
    Token, Visibility,
};

pub(super) fn make_derive(item: TokenStream) -> TokenStream {
    let structure = parse_macro_input!(item as Structure);
    let attribute_info = propagate_err!(AttributeInfo::analyze(&structure));

    let property_structure = structure.build_property_structure(&attribute_info);
    let property_impls = structure.build_property_impls(&attribute_info);

    quote! {
        #property_structure
        #property_impls
    }
    .into()
}

struct Structure {
    attributes: Vec<syn::Attribute>,
    visibility: Visibility,
    struct_token: Token![struct],
    name: syn::Ident,
    braces: syn::token::Brace,
    fields: Punctuated<syn::Field, Token![,]>,
}

impl Structure {
    fn build_property_structure(&self, attribute_info: &AttributeInfo) -> proc_macro2::TokenStream {
        let derive_info = attribute_info
            .struct_attribute_info
            .derive
            .as_ref()
            .map(|derive| quote! { #derive })
            .unwrap_or(quote! {});

        let ident = &attribute_info.struct_attribute_info.name;
        let Structure {
            ref visibility,
            ref struct_token,
            ref braces,
            ref fields,
            ..
        } = self;

        let mut tokens = quote! {
            #derive_info
            #visibility #struct_token #ident
        };

        braces.surround(&mut tokens, |tokens| {
            let wrapped_fields: Punctuated<syn::Field, Token![,]> = fields
                .clone()
                .into_iter()
                .map(|mut field| {
                    field.attrs.clear();
                    let field_name = field_name(&field);
                    let mut field_type = field.ty;

                    if let Some(field_attribute) =
                        attribute_info.field_attribute_infos.get(&field_name)
                    {
                        if let Some(use_type) = &field_attribute.use_type {
                            field_type = use_type.clone();
                        }
                    }
                    field.ty = wrap_by_option(field_type);
                    field
                })
                .collect();

            wrapped_fields.to_tokens(tokens);
        });

        tokens
    }

    fn build_property_impls(&self, attribute_info: &AttributeInfo) -> proc_macro2::TokenStream {
        let property_struct_name = &attribute_info.struct_attribute_info.name;
        let origin_struct_name = &self.name;

        let fn_unwrap_or_default = self.build_fn_unwrap_or_default(attribute_info);

        quote! {
            impl #property_struct_name {
                #fn_unwrap_or_default
            }

            impl From<#property_struct_name> for #origin_struct_name {
                fn from(val: #property_struct_name) -> Self {
                    val.unwrap_or_default()
                }
            }
        }
    }

    fn build_fn_unwrap_or_default(
        &self,
        attribute_info: &AttributeInfo,
    ) -> proc_macro2::TokenStream {
        let origin_struct_name = &self.name;

        let init_members: Punctuated<proc_macro2::TokenStream, Token![,]> = self
            .fields
            .clone()
            .into_iter()
            .map(|field| {
                let field_ident = field.ident.expect("Fields should be named");
                let field_name = field_ident.to_string();

                let mut init_line = quote! { #field_ident: self.#field_ident };

                if let Some(field_attribute) = attribute_info.field_attribute_infos.get(&field_name)
                {
                    match &field_attribute.default {
                        DefaultAssignment::CallDefault => {
                            init_line = quote! { #init_line.unwrap_or_default() }
                        }
                        DefaultAssignment::CallFunction(function) => {
                            init_line = quote! { #init_line.unwrap_or_else(#function) }
                        }
                        DefaultAssignment::Expression(expression) => {
                            init_line = quote! { #init_line.unwrap_or_else(|| #expression) }
                        }
                        DefaultAssignment::None => (),
                    }

                    if field_attribute.use_type.is_some() {
                        init_line = quote! { #init_line.into() }
                    }
                }

                init_line
            })
            .collect();

        quote! {
            fn unwrap_or_default(self) -> #origin_struct_name {
                #origin_struct_name {
                    #init_members
                }
            }
        }
    }
}

impl Parse for Structure {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            attributes: input.call(syn::Attribute::parse_outer)?,
            visibility: input.parse()?,
            struct_token: input.parse()?,
            name: input.parse()?,
            braces: braced!(content in input),
            fields: content.parse_terminated(syn::Field::parse_named, Token![,])?,
        })
    }
}

struct AttributeInfo {
    struct_attribute_info: StructAttributeInfo,
    field_attribute_infos: HashMap<String, FieldAttributeInfo>,
}

impl AttributeInfo {
    fn analyze(structure: &Structure) -> syn::Result<Self> {
        let struct_attribute_info: StructAttributeInfo = structure
            .attributes
            .clone()
            .into_iter()
            .find(Self::is_property)
            .ok_or(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Expected #[property(name(StructName))] before struct declaration",
            ))
            .and_then(|attribute| Self::attribute_inner_tokens(attribute))
            .and_then(|tokens| syn::parse2(tokens))?;

        let mut field_attribute_infos = HashMap::new();
        for field in structure.fields.clone() {
            let field_name = field_name(&field);

            let Some(attribute) = field.attrs.clone().into_iter().find(Self::is_property) else {
                continue;
            };

            field_attribute_infos.insert(
                field_name,
                Self::attribute_inner_tokens(attribute).and_then(|tokens| syn::parse2(tokens))?,
            );
        }

        Ok(Self {
            struct_attribute_info,
            field_attribute_infos,
        })
    }

    fn attribute_inner_tokens(attribute: syn::Attribute) -> syn::Result<proc_macro2::TokenStream> {
        if let syn::Meta::List(meta_list) = attribute.meta {
            Ok(meta_list.tokens)
        } else {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Expected attribute like #[property()]",
            ))
        }
    }

    fn is_property(attribute: &syn::Attribute) -> bool {
        if let syn::Meta::List(meta_list) = &attribute.meta {
            meta_list.path.is_ident("property")
        } else {
            false
        }
    }
}

struct StructAttributeInfo {
    name: syn::Ident,
    derive: Option<DeriveInfo>,
}

impl Parse for StructAttributeInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let beginning_span = input.span();
        let mut name = None;
        let mut derive = None;

        loop {
            let ident: syn::Ident = input.parse()?;

            match &*ident.to_string() {
                "name" => {
                    let content;
                    let _paren = parenthesized!(content in input);
                    name = Some(content.parse()?)
                }
                "derive" => {
                    let content;
                    derive = Some(DeriveInfo {
                        ident,
                        paren: parenthesized!(content in input),
                        traits: content.parse_terminated(syn::Path::parse, Token![,])?,
                    })
                }
                _ => {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        "Unknown attribute",
                    ))
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        let Some(name) = name else {
            return Err(syn::Error::new(
                beginning_span,
                "Expected \"name\" property for creating new struct",
            ));
        };

        Ok(Self { name, derive })
    }
}

struct DeriveInfo {
    ident: syn::Ident,
    paren: syn::token::Paren,
    traits: Punctuated<syn::Path, Token![,]>,
}

impl ToTokens for DeriveInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        Token![#](self.ident.span()).to_tokens(tokens);
        syn::token::Bracket(self.ident.span()).surround(tokens, |tokens| {
            self.ident.to_tokens(tokens);
            self.paren.surround(tokens, |tokens| {
                self.traits.to_tokens(tokens);
            });
        });
    }
}

struct FieldAttributeInfo {
    default: DefaultAssignment,
    use_type: Option<syn::Type>,
}

impl Parse for FieldAttributeInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut default = DefaultAssignment::None;
        let mut use_type = None;

        loop {
            let ident: syn::Ident = input.parse()?;

            match &*ident.to_string() {
                "default" => {
                    default = if input.peek(syn::token::Paren) {
                        let content;
                        let _paren = parenthesized!(content in input);
                        content.parse()?
                    } else {
                        DefaultAssignment::CallDefault
                    }
                }
                "use_type" => {
                    let content;
                    let _paren = parenthesized!(content in input);
                    use_type = Some(content.parse()?)
                }
                _ => {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        "Unknown attribute",
                    ))
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        Ok(Self { use_type, default })
    }
}

enum DefaultAssignment {
    CallDefault,
    CallFunction(syn::Path),
    Expression(syn::Expr),
    None,
}

impl Parse for DefaultAssignment {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(
            if input.peek(syn::Ident::peek_any) && input.peek2(Token![=]) {
                let ident: syn::Ident = input.parse()?;
                if ident != "path" {
                    return Err(syn::Error::new(proc_macro2::Span::call_site(), "Invalid declaration of default attribute. Correct usage: #[property(default(path = path::to::function))]"));
                }

                let _: Token![=] = input.parse()?;
                DefaultAssignment::CallFunction(input.parse()?)
            } else {
                DefaultAssignment::Expression(input.parse()?)
            },
        )
    }
}

fn wrap_by_option(ty: syn::Type) -> syn::Type {
    syn::Type::Path(syn::TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: None,
            segments: Punctuated::from_iter(vec![
                syn::PathSegment {
                    ident: syn::Ident::new("std", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                },
                syn::PathSegment {
                    ident: syn::Ident::new("option", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                },
                syn::PathSegment {
                    ident: syn::Ident::new("Option", proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::AngleBracketed(
                        syn::AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: syn::token::Lt(proc_macro2::Span::call_site()),
                            args: Punctuated::from_iter(vec![syn::GenericArgument::Type(ty)]),
                            gt_token: syn::token::Gt(proc_macro2::Span::call_site()),
                        },
                    ),
                },
            ]),
        },
    })
}

fn field_name(field: &syn::Field) -> String {
    field
        .ident
        .as_ref()
        .expect("Fields should be named")
        .to_string()
}
