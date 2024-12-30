use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DataEnum;
use syn::{Data, DeriveInput, Fields};

pub fn make_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = match input.data {
        Data::Enum(data_enum) => data_enum,
        _ => panic!("potom napishu"),
    };

    let mut invariant_declarations = Vec::new();
    let mut invariant_match_arms = Vec::new();

    let DataEnum { variants, .. } = data;
    for variant in variants {
        let variant_name = variant.ident;
        let (fields, field_count, field_key_values, field_names) = match variant.fields {
            Fields::Named(ref fields) => {
                let fields = fields
                    .named
                    .iter()
                    .map(|f| f.ident.clone().unwrap())
                    .collect::<Vec<_>>();
                let field_key_values = fields
                    .iter()
                    .map(|f| {
                        quote! {
                            KeyValue {
                                key: CString::new(stringify!(#f)).unwrap(),
                                value: CString::new(#f.as_str()).unwrap(),
                            }
                        }
                    })
                    .collect::<Vec<_>>();
                let field_names = fields
                    .iter()
                    .map(|f| quote! { CString::new(stringify!(#f)).unwrap() })
                    .collect::<Vec<_>>();
                (
                    quote! { { #(#fields),* } },
                    fields.len(),
                    field_key_values,
                    field_names,
                )
            }
            Fields::Unnamed(ref fields) => {
                let field_indices = (0..fields.unnamed.len())
                    .map(|i| {
                        syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site())
                    })
                    .collect::<Vec<_>>();
                let field_key_values = field_indices
                    .iter()
                    .enumerate()
                    .map(|(i, ident)| {
                        quote! {
                            KeyValue {
                                key: CString::new(format!("field{}", #i)).unwrap(),
                                value: CString::new(#ident.as_str()).unwrap(),
                            }
                        }
                    })
                    .collect::<Vec<_>>();
                let field_names = field_indices
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        quote! { CString::new(format!("field{}", #i)).unwrap() }
                    })
                    .collect::<Vec<_>>();
                (
                    quote! { ( #(ref #field_indices),* ) },
                    fields.unnamed.len(),
                    field_key_values,
                    field_names,
                )
            }
            Fields::Unit => (quote! {}, 0, Vec::new(), Vec::new()),
        };

        invariant_declarations.push(quote! {
            InvariantDeclaration {
                name: CString::new(stringify!(#variant_name)).unwrap(),
                fields: &[#(#field_names),*] as *const _,
                fields_len: #field_count,
            }
        });

        invariant_match_arms.push(quote! {
            #name::#variant_name #fields => {
                let fields = vec![#(#field_key_values),*];
                let fields_ptr = fields.as_ptr();
                let fields_len = fields.len();
                std::mem::forget(fields);
                Invariant {
                    name: CString::new(stringify!(#variant_name)).unwrap(),
                    fields: fields_ptr,
                    fields_len,
                }
            }
        });
    }

    let expanded = quote! {
        use std::ffi::CString;

        impl InteropEvent for #name {
            fn to_event(&self) -> Event {
                let invariant = match self {
                    #(#invariant_match_arms),*
                };

                let available_invariants = vec![#(#invariant_declarations),*];
                let available_invariants_ptr = available_invariants.as_ptr();
                let available_invariants_len = available_invariants.len();
                std::mem::forget(available_invariants);

                Event {
                    name: CString::new(stringify!(#name)).unwrap(),
                    invariant,
                    available_invariants: available_invariants_ptr,
                    available_invariants_len,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
