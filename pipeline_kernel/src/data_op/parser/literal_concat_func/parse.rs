use crate::{
    DataPath, DataPathSegment, FuncDataPathParam, FuncParamValue, FuncParamValuePath,
    LiteralConcatFuncParser, ParserInnerState, PathParser,
};
use watchmen_model::StdR;

impl LiteralConcatFuncParser<'_> {
    /// now a [{] encountered, so sub path will end with a [}].
    pub fn parse(&mut self) -> StdR<()> {
        let mut path_parser = PathParser {
            inner: ParserInnerState {
                full_path: self.inner.full_path,
                all_chars: self.inner.all_chars,
                char_index: self.inner.char_index,
                in_memory_chars: String::new(),
            },
            segments: vec![],
        };
        path_parser.parse_till_right_brace()?;
        let mut segments = path_parser.segments;
        if segments.is_empty() {
            // no segment, basically, it is a "{}", treated as an empty string
            self.append_param(FuncDataPathParam::Value(FuncParamValuePath {
                path: String::from(""),
                value: FuncParamValue::Str(String::from("")),
            }))
        } else if segments.len() > 1 {
            self.append_param(FuncDataPathParam::Path(DataPath {
                path: self.inner.full_path[self.inner.char_index..path_parser.inner.char_index]
                    .to_string(),
                segments,
            }))
        } else {
            match segments.pop().unwrap() {
                DataPathSegment::Plain(plain_path) => {
                    self.append_param(FuncDataPathParam::Plain(plain_path))
                }
                DataPathSegment::Func(func_path) => {
                    self.append_param(FuncDataPathParam::Func(func_path))
                }
            }
        }
        // copy char index to current state
        self.inner.char_index = path_parser.inner.char_index;

        // then check the current char, which is after the enclosing "}"
        loop {
            if let Some(char) = self.inner.current_char() {
                match char {
                    // literal end
                    '.' | ',' | ')' | '}' => break,
                    '(' => self.inner.incorrect_left_parenthesis()?,
                    '&' => self.inner.incorrect_ampersand()?,
                    // next wrapped part
                    '{' => {
                        // consume in-memory chars first
                        self.consume_in_memory_chars_as_str()?;
                        // move char index to next, after "{"
                        self.inner.move_char_index_to_next();
                        self.parse()?;
                    }
                    '\\' => self.inner.consume_potential_escape_char(),
                    // normal char, append to current chars
                    _ => self
                        .inner
                        .consume_char_into_memory_and_move_char_index_to_next(*char),
                }
            }
        }
        self.consume_in_memory_chars_as_str()?;

        Ok(())
    }
}
