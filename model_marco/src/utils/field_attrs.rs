use quote::ToTokens;
use syn::{Attribute, Meta};

pub fn get_display_value(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if let Meta::NameValue(meta) = &attr.meta {
            if meta.path.is_ident("display") {
                return Some(
                    meta.value
                        .to_token_stream()
                        .to_string()
                        .trim_matches('"')
                        .to_string(),
                );
            }
        }
    }
    None
}
