use crate::utils::{get_display_value, get_pattern, get_pattern_fn};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_serde(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let transform_fn = get_pattern_fn(get_pattern(&input.attrs));

    if let syn::Data::Enum(data_enum) = input.data {
        let variants: Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream)> = data_enum
            .variants
            .iter()
            .map(|variant| {
                let variant_name = &variant.ident;
                let variant_str = get_display_value(&variant.attrs)
                    .unwrap_or_else(|| transform_fn(&variant_name.to_string()));
                (
                    quote! {
                        #name::#variant_name => serializer.collect_str(#variant_str),
                    },
                    quote! {
                        #variant_str => Ok(#name::#variant_name),
                    },
                )
            })
            .collect();
        let ser_variants = variants.iter().map(|v| &v.0);
        let de_variants = variants.iter().map(|v| &v.1);
        let expanded = quote! {
            impl serde::Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    match self {
                        #(#ser_variants)*
                    }
                }
            }

            impl<'de> serde::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    match String::deserialize(deserializer)?.as_str() {
                        #(#de_variants)*
                        other_value => Err(serde::de::Error::custom(format!(
                            "Unsupported {} [{}].",
                            stringify!(#name),
                            &other_value
                        ))),
                    }
                }
            }
        };

        TokenStream::from(expanded)
    } else {
        panic!("`Display` can only be derived for enums");
    }
}
