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
                return match value.as_str() {
                    "kebab-lower" => VariantStrPattern::CamelCaseToKebabAndLower,
                    "kebab-upper" => VariantStrPattern::CamelCaseToKebabAndUpper,
                    "ampersand-prefix" => VariantStrPattern::AmpersandPrefix,
                    "keep-same" => VariantStrPattern::KeepSame,
                    "upper-case" => VariantStrPattern::UpperCase,
                    "lower-case" => VariantStrPattern::LowerCase,
                    _ => panic!("Unsupported pattern value [{}].", value),
                };
            }
        }
    }
    VariantStrPattern::CamelCaseToKebabAndLower
}
