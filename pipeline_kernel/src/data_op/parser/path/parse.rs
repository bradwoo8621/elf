use crate::PathParser;
use watchmen_model::StdR;

/// consume path
impl PathParser<'_> {
    /// path can contain multiple segments
    /// each segment can be,
    /// - starts with [&]: function,
    /// - not starts with [&], not contains [{}], plain,
    /// - not starts with [&], and contains [{}], function concat.
    /// and
    /// - cannot start with [().,],
    /// - cannot end with [.,],
    /// - before first [.], cannot be blank,
    /// - after last [.], cannot be blank,
    /// - for the literal concat function, functions that are not wrapped in `{}` are not allowed to appear.
    ///   e.g. [a.&len {b.len}] is not allowed.
    pub fn parse(&mut self) -> StdR<()> {
        loop {
            if let Some(char) = self.inner.current_char() {
                match char {
                    // start of function, no content before function
                    '&' => self.consume_func_path()?,
                    '(' => self.inner.incorrect_left_parenthesis()?,
                    ')' => self.inner.incorrect_right_parenthesis()?,
                    // start of sub path
                    '{' => self.consume_literal_concat_function()?,
                    '}' => self.inner.incorrect_right_brace()?,
                    // segment end
                    '.' => self.consume_in_memory_chars_as_plain_path(true)?,
                    ',' => self.inner.incorrect_comma()?,
                    // potential escape char, check next char
                    '\\' => self.inner.consume_potential_escape_char(),
                    // normal char, append to current chars
                    _ => self
                        .inner
                        .consume_char_into_memory_and_move_char_index_to_next(*char),
                };
            } else {
                // reach the end of chars,
                // consume the chars in-memory as plain path
                self.consume_in_memory_chars_as_plain_path(false)?;
                break;
            }
        }

        Ok(())
    }

    /// basically very similar to the standard parse. The differences are as follows:
    /// - parsing ends when a [}] is encountered.
    /// - if the string is completely consumed without encountering a [}], an error is reported.
    pub fn parse_till_right_brace(&mut self) -> StdR<()> {
        let index_of_left_brace = self.inner.char_index - 1;

        loop {
            if let Some(char) = self.inner.current_char() {
                match char {
                    // start of function, no content before function
                    '&' => self.consume_func_path()?,
                    '(' => self.inner.incorrect_left_parenthesis()?,
                    ')' => self.inner.incorrect_right_parenthesis()?,
                    // start of sub path
                    '{' => self.consume_literal_concat_function()?,
                    // end
                    '}' => break,
                    // segment end
                    '.' => self.consume_in_memory_chars_as_plain_path(true)?,
                    ',' => self.inner.incorrect_comma()?,
                    // potential escape char, check next char
                    '\\' => self.inner.consume_potential_escape_char(),
                    // normal char, append to current chars
                    _ => self
                        .inner
                        .consume_char_into_memory_and_move_char_index_to_next(*char),
                };
            } else {
                // reach the end of chars,
                // consume the chars in-memory as plain path
                return self.inner.incorrect_wrapped_path(index_of_left_brace);
            }
        }

        Ok(())
    }
}
