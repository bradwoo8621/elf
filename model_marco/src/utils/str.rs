pub fn camel_to_kebab(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            result.push('-');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}

pub fn first_lowercase_with_ampersand_prefix(s: &str) -> String {
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

pub enum VariantStrPattern {
    CamelCaseToKebab,
    AmpersandPrefix,
}

pub fn get_pattern_fn(pattern: VariantStrPattern) -> Box<dyn Fn(&str) -> String> {
    match pattern {
        VariantStrPattern::CamelCaseToKebab => Box::new(camel_to_kebab),
        VariantStrPattern::AmpersandPrefix => Box::new(first_lowercase_with_ampersand_prefix),
    }
}
