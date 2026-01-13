use crate::{LiteralConcatFuncParser, ParserInnerState, DataPathParser};
use elf_base::VoidR;

impl LiteralConcatFuncParser {
    /// now a [{] encountered, so sub path will end with a [}].
    pub fn parse(&mut self) -> VoidR {
        let mut path_parser = DataPathParser {
            inner: ParserInnerState::new_at_current_char(&self.inner),
            segments: vec![],
        };
        path_parser.parse_till_right_brace()?;
        path_parser.hand_back_to_literal_concat(self);

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
            } else {
                // reach the end, no char anymore
                break;
            }
        }

        self.consume_in_memory_chars_as_str()?;

        Ok(())
    }
}
