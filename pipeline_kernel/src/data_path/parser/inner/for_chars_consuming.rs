use crate::ParserInnerState;

/// consume chars
impl ParserInnerState {
    /// append given char to in-memory chars
    /// and move char index to next
    pub fn consume_char_into_memory_and_keep_char_index(&mut self, char: char) {
        self.collect_char_into_memory(char);
    }

    /// append given char to in-memory chars
    /// and move char index to next
    pub fn consume_char_into_memory_and_move_char_index_to_next(&mut self, char: char) {
        self.collect_char_into_memory(char);
        self.move_char_index_to_next();
    }

    /// check the given char can be escaped or not
    /// if yes, append the escaped char to given str, move char index to index after the escaped char.
    /// otherwise append char '\' to given str, move char index to index after [\].
    pub fn consume_potential_escape_char(&mut self) {
        // current char index is point to the char "\", move to next
        self.move_char_index_to_next();

        if let Some(next_c) = self.current_char() {
            match next_c {
                '.' | ',' | '(' | ')' | '{' | '}' | '&' => {
                    self.consume_char_into_memory_and_move_char_index_to_next(*next_c);
                }
                't' => self.consume_char_into_memory_and_move_char_index_to_next('\t'),
                'r' => self.consume_char_into_memory_and_move_char_index_to_next('\r'),
                'n' => self.consume_char_into_memory_and_move_char_index_to_next('\n'),
                _ => {
                    self.consume_char_into_memory_and_keep_char_index('\\');
                }
            }
        } else {
            self.consume_char_into_memory_and_keep_char_index('\\');
        }
    }
}
