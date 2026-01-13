use crate::{DataPathSegment, FuncDataPath, DataPathFuncParser, ParserInnerState, DataPathParser, PathStr};
use elf_base::{StdR, VoidR};
use elf_model::VariablePredefineFunctions;

/// consume function
impl DataPathParser {
    /// consume in-memory chars as a function name.
    /// the in-memory chars never be empty, at least a [&] in it.
    /// and clear in-memory chars if consumed, will not move char index
    fn consume_in_memory_chars_as_func_name(&mut self) -> StdR<VariablePredefineFunctions> {
        let in_memory_chars_count = self.inner.in_memory_chars_count();
        if in_memory_chars_count <= 1 {
            self.incorrect_empty_function_name()
        } else if let Some(func) =
            VariablePredefineFunctions::try_parse(self.inner.in_memory_chars())
        {
            self.inner.clear_in_memory_chars();
            Ok(func)
        } else {
            self.incorrect_function_name(in_memory_chars_count)
        }
    }

    /// try to consume function path, and return the char index after the function path.
    /// - if function has no param and no parenthesis, it stops at end of full path or any char not [a-zA-Z0-9_],
    ///   then return
    /// > any chars before function is not allowed.
    pub fn consume_func_path(&mut self) -> VoidR {
        let inner = &mut self.inner;

        if inner.in_memory_chars_is_not_empty() {
            return self.inner.incorrect_ampersand();
        }

        // index of `&`, which is start of function
        let start_index_of_func = inner.current_char_index();
        inner.consume_char_into_memory_and_move_char_index_to_next('&');

        let mut whitespace_met = false;
        let mut will_start_params = false;
        loop {
            if let Some(char) = self.inner.current_char() {
                match char {
                    // function name can be A-Za-z0-9_
                    'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => {
                        if whitespace_met {
                            self.incorrect_function_name_contains_whitespace(start_index_of_func)?;
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
                    _ => self.incorrect_function_name_char(*char)?,
                }
            } else {
                // reach the end, no char anymore
                break;
            }
        }

        let func = self.consume_in_memory_chars_as_func_name()?;
        // check has context and allowable
        let has_context = self.has_segment();
        if !func.require_context() && has_context {
            // has context and context disallowed, error
            return self.incorrect_function_has_context(start_index_of_func);
        }

        if will_start_params {
            // continue parsing function
            let mut func_parser = DataPathFuncParser {
                inner: ParserInnerState::new_at_next_char(&self.inner),
                start_char_index_of_func: start_index_of_func,
                func,
                params: vec![],
                with_context: self.has_segment(),
            };
            func_parser.parse()?;
            // hand back
            // copy char index to current state
            self.inner
                .move_char_index_to(func_parser.inner.current_char_index());
            // create and append the function to segments
            self.append_segment(DataPathSegment::Func(FuncDataPath {
                path: self
                    .inner
                    .create_path_str_exclude_current(start_index_of_func),
                func: func_parser.func,
                params: if func_parser.params.is_empty() {
                    None
                } else {
                    Some(func_parser.params)
                },
            }));
        } else {
            // no params followed
            let min_param_count = func.min_param_count();
            if min_param_count > 0 || (!has_context && func.require_context()) {
                // at least 1 parameter required,
                // or has no context and context required
                // error
                return self.incorrect_function_no_param(&func);
            } else {
                // func with has no param
                self.append_segment(DataPathSegment::Func(FuncDataPath {
                    path: PathStr::part_of_chars(
                        self.inner.all_chars().clone(),
                        start_index_of_func,
                        start_index_of_func + func.to_string().chars().count(),
                    ),
                    func,
                    params: None,
                }));
            }
        }

        Ok(())
    }
}
