use rand::{rng, Rng};
use std::sync::OnceLock;

pub trait RandomStr {
    fn random(len: usize) -> String;

    fn random_16() -> String {
        Self::random(16)
    }

    fn random_32() -> String {
        Self::random(32)
    }
}

static ALL_PRINTABLE_ASCII: OnceLock<Vec<char>> = OnceLock::new();

impl RandomStr for String {
    fn random(len: usize) -> String {
        let all_printable_ascii = ALL_PRINTABLE_ASCII
            .get_or_init(|| (32u8..=126u8).map(|c| c as char).collect::<Vec<char>>());

        let mut rng = rng();
        (0..len)
            .map(|_| {
                let idx = rng.random_range(0..all_printable_ascii.len());
                all_printable_ascii.get(idx).unwrap()
            })
            .collect()
    }
}
