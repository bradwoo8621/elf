use crate::utils::VariantStrPattern;
use quote::ToTokens;
use syn::{Attribute, Meta};

pub fn get_pattern(attrs: &Vec<Attribute>) -> VariantStrPattern {
    for attr in attrs {
        if let Meta::NameValue(meta) = &attr.meta {
            if meta.path.is_ident("pattern") {
                let value = (&meta.value)
                    .to_token_stream()
                    .to_string()
                    .trim_matches('"')
                    .to_string();
                if value == "kebab" {
                    return VariantStrPattern::CamelCaseToKebab;
                } else if value == "ampersand-prefix" {
                    return VariantStrPattern::AmpersandPrefix;
                } else {
                    panic!("Unsupported pattern value [{}].", value)
                }
            }
        }
    }
    VariantStrPattern::CamelCaseToKebab
}
