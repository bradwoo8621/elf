use crate::model_adapt::adapt_to::AdaptTo;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Fields};

mod adapt_to;

pub fn model_adapt(attr: TokenStream, item: TokenStream) -> TokenStream {
    if attr.is_empty() {
        panic!(
            "This macro only works on attributes [bdm, storable, audit, opt_lock, tuple, tenant_based, user_based, last_visit]."
        )
    }

    let attr_str = attr.to_string();
    let meta: Vec<&str> = attr_str.split(",").map(|s| s.trim()).collect();
    let mut adapt_to = AdaptTo::new();
    for (_, target) in meta.iter().enumerate() {
        adapt_to.set(*target);
    }

    let input = parse_macro_input!(item as DeriveInput);
    let vis = &input.vis.to_token_stream();
    let input_name = &input.ident;
    match input.data {
        syn::Data::Struct(s) => match s.fields {
            Fields::Named(named_fields) => {
                let existing_fields = adapt_to.rebuild_existing_fields(&named_fields.named);
                // modifications
                let attributes = adapt_to.attributes();
                let new_fields = adapt_to.fields();
                let traits = adapt_to.traits(input_name);
                let expanded = quote! {
                    #attributes
                    #vis struct #input_name {
                        #existing_fields
                        #new_fields
                    }

                    #traits
                };
                TokenStream::from(expanded)
            }
            Fields::Unnamed(_) => {
                panic!("This macro only works on structs with name field.")
            }
            Fields::Unit => {
                panic!("This macro only works on structs with name field.")
            }
        },
        syn::Data::Enum(e) => {
            if !adapt_to.suitable_for_enum() {
                panic!("Enums can only adapt to [bdm] or [storable].");
            }
            let variants = e.variants.to_token_stream();
            // modifications
            let attributes = adapt_to.attributes();
            let traits = adapt_to.traits(input_name);
            let expanded = quote! {
                #attributes
                #vis enum #input_name {
                    #variants
                }

                #traits
            };
            TokenStream::from(expanded)
        }
        _ => panic!("This macro only works on structs."),
    }
}
