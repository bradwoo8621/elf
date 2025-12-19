use crate::{DataPathSegment, FuncDataPath, FuncParser, ParserInnerState, PathParser};
use watchmen_model::{StdR, VariablePredefineFunctions};

/// consume function
impl PathParser<'_> {
    /// consume in-memory chars as a function name.
    /// the in-memory chars never be empty, at least a [{] in it.
    /// and clear in-memory chars if consumed, will not move char index
    fn consume_in_memory_chars_as_func_name(&mut self) -> StdR<VariablePredefineFunctions> {
        let in_memory_chars = &self.inner.in_memory_chars;
        if in_memory_chars.len() <= 1 {
            self.inner
                .incorrect_empty_function_name(self.inner.char_index - 1)
        } else if let Some(func) = VariablePredefineFunctions::try_parse(in_memory_chars) {
            self.inner.clear_in_memory_chars();
            Ok(func)
        } else {
            self.inner.incorrect_function_name(
                self.inner.char_index - in_memory_chars.len(),
                self.inner.char_index,
            )
        }
    }

    /// try to consume function path, and return the char index after the function path.
    /// - if function has no param and no parenthesis, it stops at end of full path or any char not [a-zA-Z0-9_],
    ///   then return
    /// > any chars before function is not allowed.
    pub fn consume_func_path(&mut self) -> StdR<()> {
        let inner = &mut self.inner;

        if inner.in_memory_chars_is_not_empty() {
            return self.inner.incorrect_ampersand();
        }

        let start_index_of_func = inner.char_index;
        inner.consume_char_into_memory_and_move_char_index_to_next('&');

        let mut whitespace_met = false;
        let mut will_start_params = false;
        loop {
            if let Some(char) = self.inner.current_char() {
                match char {
                    // function name can be A-Za-z0-9_
                    'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => {
                        if whitespace_met {
                            self.inner.incorrect_function_name_contains_whitespace(
                                start_index_of_func,
                                self.inner.char_index + 1,
                            )?;
                        } else {
                            self.inner
                                .consume_char_into_memory_and_move_char_index_to_next(*char)
                        }
                    }
                    // \s, ignore.
                    // in python version, whitespaces after function name is allowed, compatible logic here
                    ' ' | '\t' | '\r' | '\n' | '\x0C' | '\x0B' => {
                        whitespace_met = true;
                        self.inner.move_char_index_to_next();
                    }
                    '&' => self.inner.incorrect_ampersand()?,
                    // end of function name, start function parameters
                    '(' => {
                        will_start_params = true;
                        break;
                    }
                    ')' => self.inner.incorrect_right_parenthesis()?,
                    '{' => self.inner.incorrect_left_brace()?,
                    '}' => self.inner.incorrect_right_brace()?,
                    // end of function name, if it is in segment slot
                    '.' => break,
                    // end of function name, if it is in parameter slot
                    ',' => break,
                    _ => self.inner.incorrect_function_name_char(*char)?,
                }
            } else {
                // reach the end, no char anymore
                break;
            }
        }

        let func = self.consume_in_memory_chars_as_func_name()?;
        // check has context and allowable
        let has_context = !self.segments.is_empty();
        if !func.require_context() && has_context {
            return self
                .inner
                .incorrect_function_has_context(start_index_of_func, self.inner.char_index);
        }

        if will_start_params {
            // continue parsing function
            let mut func_parser = FuncParser {
                inner: ParserInnerState {
                    full_path: self.inner.full_path,
                    all_chars: self.inner.all_chars,
                    // skip the "("
                    char_index: self.inner.char_index + 1,
                    in_memory_chars: String::new(),
                },
                start_char_index_of_func: start_index_of_func,
                func,
                params: vec![],
                with_context: !self.segments.is_empty(),
            };
            func_parser.parse()?;
            // hand back
            // copy char index to current state
            self.inner.char_index = func_parser.inner.char_index;
            // create and append the function to segments
            self.append_segment(DataPathSegment::Func(FuncDataPath {
                path: self.inner.full_path[start_index_of_func..self.inner.char_index].to_string(),
                func: func_parser.func,
                params: if func_parser.params.is_empty() {
                    None
                } else {
                    Some(func_parser.params)
                },
            }));
        } else {
            // no params followed, then raise error when
            // - if at least 1 parameter required,
            // - has no context and not context disallowed, which means context
            let min_param_count = func.min_param_count();
            if min_param_count > 0 || !has_context {
                return self.inner.incorrect_function_no_param(
                    self.inner.char_index - func.to_string().len(),
                    self.inner.char_index,
                );
            }

            // func with has no param
            self.append_segment(DataPathSegment::Func(FuncDataPath {
                path: func.to_string(),
                func,
                params: None,
            }))
        }

        Ok(())
    }
}
