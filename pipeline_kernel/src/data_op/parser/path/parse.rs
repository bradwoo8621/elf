use crate::PathParser;
use watchmen_model::StdR;

/// consume path
impl PathParser<'_> {
    /// try to consume in-memory chars when dot met.
    /// not allowed:
    /// - continuous dots,
    /// - start of path,
    ///   - after left parenthesis,
    ///   - after left brace,
    ///   - after ampersand,
    ///   - after comma,
    ///   - not content before.
    fn consume_in_memory_chars_before_dot(&mut self) -> StdR<()> {
        if self.inner.in_memory_chars_is_empty() {
            // check the previous char
            if let Some(previous_char) = self.inner.previous_char() {
                match previous_char {
                    // dot is not allowed as start of path
                    '.' | '(' | '{' | '&' | ',' => self.inner.incorrect_dot(),
                    _ => {
                        // previous chars already consumed, simply move index to next
                        self.inner.move_char_index_to_next();
                        Ok(())
                    }
                }
            } else {
                // no char before dot, dot cannot be the first char
                self.inner.incorrect_dot()
            }
        } else {
            self.consume_in_memory_chars_as_plain_path(true)
        }
    }

    /// try to consume in-memory chars when path end met.
    /// now allowed:
    /// - after dot,
    /// - after left parenthesis, handled by []
    /// - after left brace,
    /// - after ampersand,
    /// - after comma,
    /// - no content before
    fn consume_in_memory_chars_before_end(&mut self) -> StdR<()> {
        if self.inner.in_memory_chars_is_empty() {
            // check the previous char
            if let Some(previous_char) = self.inner.previous_char() {
                match previous_char {
                    // previous char is start of something, and not ends yet
                    '.' | '(' | '{' | '&' | ',' => {
                        self.inner.incorrect_char_at_previous_index(previous_char)
                    }
                    // previous chars already consumed
                    _ => Ok(()),
                }
            } else {
                // no char before end, empty path is not allowed
                self.inner.incorrect_empty_path()
            }
        } else {
            self.consume_in_memory_chars_as_plain_path(false)
        }
    }

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
                    '.' => self.consume_in_memory_chars_before_dot()?,
                    ',' => self.inner.incorrect_comma()?,
                    // potential escape char, check next char
                    '\\' => self.inner.consume_potential_escape_char(),
                    // normal char, append to current chars
                    _ => self
                        .inner
                        .consume_char_into_memory_and_move_char_index_to_next(*char),
                };
            } else {
                // reach the end, no char anymore
                // consume the chars in-memory as plain path
                self.consume_in_memory_chars_before_end()?;
                break;
            }
        }

        Ok(())
    }

    /// called by [parse_till_right_brace] only.
    fn consume_in_memory_chars_before_right_brace(&mut self) -> StdR<()> {
        if self.inner.in_memory_chars_is_empty() {
            // check the previous char
            if let Some(previous_char) = self.inner.previous_char() {
                match previous_char {
                    // previous char is start of something, and not ends yet
                    '.' | '(' | '&' | ',' => self.inner.incorrect_right_brace(),
                    '{' | _ => {
                        // is an empty wrapped path or previous chars already consumed
                        // simply move index to next
                        self.inner.move_char_index_to_next();
                        Ok(())
                    }
                }
            } else {
                // no char before right brace, it cannot be the first char
                // never happen there always a "{" to trigger the caller function
                self.inner.incorrect_right_brace()
            }
        } else {
            self.consume_in_memory_chars_as_plain_path(true)
        }
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
                    '}' => {
                        self.consume_in_memory_chars_before_right_brace()?;
                        break;
                    }
                    // segment end
                    '.' => self.consume_in_memory_chars_before_dot()?,
                    ',' => self.inner.incorrect_comma()?,
                    // potential escape char, check next char
                    '\\' => self.inner.consume_potential_escape_char(),
                    // normal char, append to current chars
                    _ => self
                        .inner
                        .consume_char_into_memory_and_move_char_index_to_next(*char),
                };
            } else {
                // reach the end, no char anymore
                // consume the chars in-memory as plain path
                return self.inner.incorrect_wrapped_path(index_of_left_brace);
            }
        }

        Ok(())
    }
}
