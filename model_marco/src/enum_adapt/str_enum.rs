use crate::utils::{get_display_value, get_pattern, get_pattern_fn};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_str_enum(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;

    if let syn::Data::Enum(data_enum) = input.data {
        let transform_fn = get_pattern_fn(get_pattern(&input.attrs));
        let variants: Vec<(&Ident, String)> = data_enum
            .variants
            .iter()
            .map(|variant| {
                let variant_name = &variant.ident;
                (
                    variant_name,
                    get_display_value(&variant.attrs)
                        .unwrap_or_else(|| transform_fn(&variant_name.to_string())),
                )
            })
            .collect();
        let variants1 = variants.iter().map(|(variant_name, variant_str)| {
            quote! {
                #variant_str => Ok(#name::#variant_name),
            }
        });
        let variants2 = variants.iter().map(|(variant_name, variant_str)| {
            quote! {
                #variant_str => Some(#name::#variant_name),
            }
        });

        let expanded = quote! {
            impl #name {
                pub fn parse<S>(str: S) -> StdR<Self>
                where
                    S: Into<String>,
                {
                    match str.into().as_str() {
                        #(#variants1)*
                        s => StdErrCode::StrEnumParse.msg(format!("Cannot parse string[{}] to {}.", s, stringify!(#name))),
                    }
                }

                pub fn try_parse<S>(str: S) -> Option<Self>
                where
                    S: Into<String>,
                {
                    match str.into().as_str() {
                        #(#variants2)*
                        _ => None,
                    }
                }
            }
        };

        TokenStream::from(expanded)
    } else {
        panic!("`StrEnum` can only be derived for enums");
    }
}
