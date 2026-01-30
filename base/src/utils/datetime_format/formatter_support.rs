use std::collections::HashMap;

pub struct DateTimeFormatterSupport {
    pub format: String,
    pub len: usize,
    /// has millisecond ([%f, %3f]) or not
    pub has_milli: bool,
    /// has timezone ([%z]) or not
    pub has_tz: bool,
}

impl DateTimeFormatterSupport {
    // never check the format is valid or not
    pub fn build(format: &String) -> Self {
        let mut len: usize = 0;
        let mut has_millisecond = false;
        let mut has_timezone = false;
        let mut parsed_format = String::new();

        for parts in format.split('%') {
            match parts {
                "Y" => {
                    len += 4;
                    parsed_format.push('%');
                    parsed_format.push('Y')
                }
                "y" | "m" | "d" | "H" | "M" | "S" => {
                    len += 2;
                    parsed_format.push('%');
                    parsed_format.push(parts.chars().nth(0).unwrap());
                }
                "f" | "3f" => {
                    len += 3;
                    has_millisecond = true;
                    parsed_format.push('%');
                    parsed_format.push('3');
                    parsed_format.push('f')
                }
                "z" => {
                    len += 5;
                    has_timezone = true;
                    parsed_format.push('%');
                    parsed_format.push('z')
                }
                // not supported format, ignored, just make the length to 100
                "C" | "q" | "B" | "b" | "h" | "e" | "A" | "a" | "w" | "u" | "U" | "W" | "G"
                | "g" | "V" | "j" | "D" | "x" | "F" | "v" | "k" | "I" | "l" | "P" | "p" | ".f"
                | ".3f" | ".6f" | ".9f" | "6f" | "9f" | "R" | "T" | "X" | "r" | "Z" | ":z"
                | "::z" | ":::z" | "#z" | "c" | "+" | "s" => len += 100,
                // other chars, ignore
                _ => {}
            }
        }

        Self {
            format: parsed_format,
            len,
            has_milli: has_millisecond,
            has_tz: has_timezone,
        }
    }

    /// parse given format string
    pub fn build_map(formats: Vec<String>) -> HashMap<usize, Vec<DateTimeFormatterSupport>> {
        formats
            .iter()
            .map(|s| DateTimeFormatterSupport::build(s))
            .fold(HashMap::new(), |mut map, fs| {
                map.entry(fs.len).or_insert_with(Vec::new).push(fs);
                map
            })
    }

    /// get digit and plus chars of given string
    /// returns collected chars and count
    pub fn valid_part(str: &String) -> (String, usize) {
        let mut s = String::new();
        for char in str.chars() {
            if char.is_ascii_digit() || char == '+' {
                s.push(char);
            }
        }
        let count = s.chars().count();
        (s, count)
    }
}
