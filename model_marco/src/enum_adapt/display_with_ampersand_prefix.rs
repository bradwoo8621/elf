use crate::enum_adapt::utils::get_display_value;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn first_lowercase_with_ampersand(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => format!(
            "&{}{}",
            first.to_ascii_lowercase(),
            chars.collect::<String>()
        ),
        None => String::from("&"),
    }
}

pub fn impl_display_with_ampersand_prefix(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;

    if let syn::Data::Enum(data_enum) = input.data {
        let variants = data_enum.variants.into_iter().map(|variant| {
            let variant_name = variant.ident;
            let variant_str = get_display_value(&variant.attrs)
                .unwrap_or_else(|| first_lowercase_with_ampersand(&variant_name.to_string()));
            quote! {
                #name::#variant_name => write!(f, "{}", #variant_str),
            }
        });
        let expanded = quote! {
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(#variants)*
                    }
                }
            }
        };

        TokenStream::from(expanded)
    } else {
        panic!("`Display` can only be derived for enums");
    }
}
