use config::{FileStoredFormat, Format, Map, Value, ValueKind};
use dotenvy::Iter;
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct EnvFile;

/// base on [dotenv.org](https://www.dotenv.org/docs/languages/rust),
/// it is use [dotenvy] as the underlay implementation.
/// so, [dotenvy] is also used as the underlying implementation here.
/// the difference is that there will be no intersection with the system environment variables here.
/// it simply reads the given content into memory.
impl Format for EnvFile {
    /// use [dotenvy::Iter] to parse, but not write to os env.
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<Map<String, Value>, Box<dyn std::error::Error + Send + Sync>> {
        let mut map: Map<String, Value> = Map::new();

        let mut content = text;
        // remove bom, from Iter::remove_bom, but function is private
        if content.len() >= 3 && content.as_bytes().starts_with(&[0xEF, 0xBB, 0xBF]) {
            content = &content[3..];
        }

        let cursor = Cursor::new(content);
        let iter = Iter::new(cursor);
        for item in iter {
            let (key, value) = item?;
            map.insert(key, Value::new(uri, ValueKind::String(value)));
        }

        Ok(map)
    }
}

// A slice of extensions associated to this format, when an extension
// is omitted from a file source, these will be tried implicitly:
impl FileStoredFormat for EnvFile {
    fn file_extensions(&self) -> &'static [&'static str] {
        &["env"]
    }
}
