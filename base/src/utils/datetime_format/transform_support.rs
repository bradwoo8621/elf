pub struct DateTimeFormatTransformSupport;

impl DateTimeFormatTransformSupport {
    pub fn transform(format: &String) -> String {
        let mut transformed = vec![];
        for char in format.chars() {
            let transformed_char = match char {
                'Y' => 'Y', // 4 digits year
                'y' => 'y', // 2 digits year
                'M' => 'm', // 2 digits month
                'D' => 'd', // 2 digits day of month
                'h' => 'H', // 2 digits hour, 00 - 23
                'H' => 'I', // 2 digits hour, 01 - 12
                'm' => 'M', // 2 digits minute
                's' => 'S', // 2 digits second
                'W' => 'A', // Monday - Sunday
                'w' => 'a', // Mon - Sun
                'B' => 'B', // January - December
                'b' => 'b', // Jan - Dec
                'p' => 'p', // AM/PM
                _ => {
                    transformed.push(char);
                    continue;
                }
            };
            transformed.push('%');
            transformed.push(transformed_char);
        }
        transformed.into_iter().collect()
    }
}
