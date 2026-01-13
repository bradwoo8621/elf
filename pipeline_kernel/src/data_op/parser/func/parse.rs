use crate::{
    AnyFuncParser, FuncDataPathParam, FuncParamValue, FuncParamValuePath, FuncParser,
    ParserInnerState, PathParser,
};
use elf_base::VoidR;

struct FuncParserDelegate<'a> {
    parser: &'a mut FuncParser,
    param_start_char_index: usize,
}

impl AnyFuncParser for FuncParserDelegate<'_> {
    fn append_param(&mut self, param: FuncDataPathParam) {
        self.parser.params.push(param);
    }

    fn param_start_char_index(&self) -> usize {
        self.param_start_char_index
    }

    fn move_char_index_to(&mut self, char_index: usize) {
        self.parser.inner.move_char_index_to(char_index);
    }
}

impl FuncParser {
    /// start parsing parameter.
    /// the function parameter parsing starts when
    /// - & encountered,
    /// - char not whitespaces, and not one of [,()],
    /// which means there is at least one segment will be parsed from path parser.
    fn parse_param(&mut self, param_start_char_index: usize) -> VoidR {
        let mut path_parser = PathParser {
            inner: ParserInnerState::new_at_current_char_and_copy_in_memory_chars(&mut self.inner),
            segments: vec![],
        };
        path_parser.parse_till_param_end(param_start_char_index)?;
        let delegate = &mut FuncParserDelegate {
            parser: self,
            param_start_char_index,
        };
        path_parser.hand_back_to_func(delegate);

        Ok(())
    }

    /// check if there are tailing whitespaces when parameter already parsed.
    /// if there are, raise error.
    fn check_tailing_whitespaces_when_param_parsed(&self) -> VoidR {
        if self.inner.in_memory_chars_is_not_empty() {
            // never happens, all chars should be consumed in parse_param
            self.incorrect_function_param_tailing_whitespaces(self.inner.in_memory_chars_count())
        } else {
            Ok(())
        }
    }

    /// consume in-memory chars as blank string parameter.
    /// and clear in-memory chars
    fn append_blank_param(&mut self) -> VoidR {
        self.params
            .push(FuncDataPathParam::Value(FuncParamValuePath {
                path: self.inner.create_path_str_of_in_memory_chars(),
                value: FuncParamValue::Str(self.inner.clone_in_memory_chars()),
            }));

        self.inner.clear_in_memory_chars();

        Ok(())
    }

    /// consume in-memory chars as none parameter.
    /// and clear in-memory chars
    ///
    /// note the in-memory chars might not empty, it is treated as none here.
    fn append_none_param(&mut self) -> VoidR {
        self.params
            .push(FuncDataPathParam::Value(FuncParamValuePath {
                path: self.inner.create_path_str_of_in_memory_chars(),
                value: FuncParamValue::None,
            }));

        self.inner.clear_in_memory_chars();

        Ok(())
    }

    /// append whitespaces in memory as context parameter
    /// - if there are chars in memory,
    ///   - if function allow blank context, append as blank string parameter,
    ///   - else if function allow none context, append as none parameter,
    ///   - else raise error,
    /// - if no char in memory,
    ///   - if function allow none context, append as none parameter,
    ///   - else raise error.
    fn append_whitespaces_as_context(&mut self) -> VoidR {
        if self.inner.in_memory_chars_is_not_empty() {
            // there are chars in memory
            if self.func.allow_blank_context() {
                // context allows string
                self.append_blank_param()
            } else if self.func.allow_none_context() {
                self.append_none_param()
            } else {
                // context does not allow none, raise error
                self.incorrect_function_invalid_context(self.inner.in_memory_chars_count())
            }
        } else {
            // no char in memory, check none is acceptable or not
            if self.func.allow_none_context() {
                self.append_none_param()
            } else {
                // context does not allow none, raise error
                self.incorrect_function_invalid_context(self.inner.in_memory_chars_count())
            }
        }
    }

    /// append whitespaces in memory as parameter at given index
    /// - if there are chars in memory,
    ///   - if function allow blank parameter at given index, append as blank string parameter,
    ///   - else if function allow none parameter at given index, append as none parameter,
    ///   - else raise error,
    /// - if no char in memory,
    ///   - if function allow none parameter at given index, append as none parameter,
    ///   - else raise error.
    fn append_whitespaces_as_parameter(&mut self, real_param_index: usize) -> VoidR {
        if self.inner.in_memory_chars_is_not_empty() {
            // there are chars in memory
            if self.func.allow_blank_param(real_param_index) {
                // context allows string
                self.append_blank_param()
            } else if self.func.allow_none_param(real_param_index) {
                self.append_none_param()
            } else {
                // context does not allow none, raise error
                self.incorrect_function_invalid_context(self.inner.in_memory_chars_count())
            }
        } else {
            // no char in memory, check none is acceptable or not
            if self.func.allow_none_param(real_param_index) {
                self.append_none_param()
            } else {
                // context does not allow none, raise error
                self.incorrect_function_invalid_context(self.inner.in_memory_chars_count())
            }
        }
    }

    /// end parameter parsing when no parameter parsed yet.
    /// - if parsed count is not 0, check tailing in-memory chars as error.,
    /// - if no parameter parsed yet,
    ///   - if not with context and function require context, try to append in-memory chars as context,
    ///   - else try to append in-memory chars as first parameter.
    fn end_param_at_0(&mut self) -> VoidR {
        let parsed_count = self.params.len();
        if parsed_count != 0 {
            return self.check_tailing_whitespaces_when_param_parsed();
        }

        if !self.with_context && self.func.require_context() {
            // current is context
            self.append_whitespaces_as_context()
        } else {
            // current is parameter
            let max_param_count = self.func.max_param_count();
            if let Some(max_count) = max_param_count {
                if max_count == 0 {
                    // no parameter allowed
                    // simply ignore in-memory chars and clear it
                    // they are just whitespaces between left/right parenthesis
                    self.inner.clear_in_memory_chars();
                    return Ok(());
                }
            }

            // at least one parameter required, or no limit
            self.append_whitespaces_as_parameter(0)
        }
    }

    fn end_param_at_not_0(
        &mut self,
        index_of_left_parenthesis: usize,
        param_index: usize,
    ) -> VoidR {
        let real_param_index = self.real_param_index(param_index);
        let max_param_count = self.func.max_param_count();
        if let Some(max_count) = max_param_count {
            if max_count <= real_param_index {
                // current parameter is not allowed, over the max param count
                return self
                    .incorrect_function_param_over_max_count(index_of_left_parenthesis, max_count);
            }
        }

        self.append_whitespaces_as_parameter(real_param_index)
    }

    /// end param parse when one of [,)] detected
    /// check parsed parameters count, if given param index already parsed (by [parse_param]),
    /// then do nothing.
    /// otherwise check the in-memory chars,
    /// - not empty,
    ///   - check function accept none in current param index or not
    ///   - check function accept blank string in current param index or not
    ///   - check function accept empty string in current param index or not
    ///   - none of above accepted, raise error (since blank plain path is not accepted).
    /// - empty,
    ///   - check function accept none in current param index or not
    ///   - check function accept empty string in current param index or not
    ///   - none of above accepted, raise error (since blank plain path is not accepted).
    /// move char index to next at last
    fn end_param(&mut self, index_of_left_parenthesis: usize, param_index: usize) -> VoidR {
        let parsed_count = self.params.len();
        if param_index == 0 && parsed_count != 1 {
            // no parameter parsed, current is the first parameter.
            self.end_param_at_0()?
        } else if parsed_count <= param_index {
            // current parameter not parsed yet
            self.end_param_at_not_0(index_of_left_parenthesis, param_index)?
        } else {
            // already parsed, do nothing
            self.check_tailing_whitespaces_when_param_parsed()?;
        }

        // move to next char, skip the ")" or ","
        self.inner.move_char_index_to_next();

        Ok(())
    }

    fn end_param_before_right_parenthesis(
        &mut self,
        index_of_left_parenthesis: usize,
        param_index: usize,
    ) -> VoidR {
        self.check_param_count_before_right_parenthesis(index_of_left_parenthesis, param_index)?;
        self.end_param(index_of_left_parenthesis, param_index)
    }

    /// it is not the last parameter of function, there is one more parameter will be parsed after comma.
    fn end_param_before_comma(
        &mut self,
        index_of_left_parenthesis: usize,
        param_index: usize,
    ) -> VoidR {
        self.check_param_count_before_comma(index_of_left_parenthesis, param_index)?;
        self.end_param(index_of_left_parenthesis, param_index)
    }

    /// - check min param count,
    /// - check max param count
    fn final_param_count_check(&mut self, index_of_left_parenthesis: usize) -> VoidR {
        let parsed_count = self.params.len();
        let parsed_count = if self.with_context || !self.func.require_context() {
            // with context or no context required
            // all parsed are parameters
            parsed_count
        } else if parsed_count == 0 {
            // no context and no parameter parsed, but context required
            return self.incorrect_function_has_no_context();
        } else {
            // no context, context required, at least one parameter parsed
            // the first parsed is context, rest are parameters
            parsed_count - 1
        };

        let min_param_count = self.func.min_param_count();
        if parsed_count < min_param_count {
            return self.incorrect_function_param_below_min_count(
                index_of_left_parenthesis,
                min_param_count,
            );
        }

        if let Some(max_param_count) = self.func.max_param_count() {
            if parsed_count > max_param_count {
                return self.incorrect_function_param_over_max_count(
                    index_of_left_parenthesis,
                    max_param_count,
                );
            }
        }

        Ok(())
    }

    /// now a function name and [(] encountered, so parameters part will end with a [)].
    ///
    /// the parameter can contain whitespaces,
    /// so it is hard to determine that the prefix/suffix whitespaces are valid or not.
    /// basically, current whitespaces are not allowed
    /// - before [&], e.g. [ &now] is invalid,
    /// - after [.], e.g. [a. ] is invalid,
    /// and suffix whitespaces in literal concat function will be collected as blank string value,
    /// it needs to be ignored.
    ///
    /// so when start parsing parameter, collect whitespaces into memory first,
    /// when there are in-memory whitespace chars collected, and
    /// - if there is a [&] determined, raise error,
    /// - if there is one of [,)] determined, means no parameter lacked, raise error,
    /// - if there is a [(] determined, raise error,
    /// - otherwise copy in-memory chars to path parser to continue.
    pub fn parse(&mut self) -> VoidR {
        let index_of_left_parenthesis = self.inner.previous_char_index() as usize;

        // total parsed parameter count, include context if not with context currently
        let mut param_index: usize = 0;
        let mut param_start_char_index = self.inner.current_char_index();
        loop {
            if let Some(char) = self.inner.current_char() {
                match char {
                    '&' => {
                        // ignore the whitespaces before function
                        self.inner.clear_in_memory_chars();
                        self.parse_param(param_start_char_index)?;
                    }
                    '(' => self.inner.incorrect_left_parenthesis()?,
                    // end of parameters
                    ')' => {
                        self.end_param_before_right_parenthesis(
                            index_of_left_parenthesis,
                            param_index,
                        )?;
                        // will break, no need to increase param_index
                        // param_index += 1;
                        break;
                    }
                    // end of parameter
                    ',' => {
                        self.end_param_before_comma(index_of_left_parenthesis, param_index)?;
                        param_index += 1;
                        // char index already moved to next in end_param
                        param_start_char_index = self.inner.current_char_index();
                    }
                    // \s, ignore.
                    // in python version, whitespaces after function name is allowed, compatible logic here
                    ' ' | '\t' | '\r' | '\n' | '\x0C' | '\x0B' => self
                        .inner
                        .consume_char_into_memory_and_move_char_index_to_next(*char),
                    // other char
                    _ => self.parse_param(param_start_char_index)?,
                };
            } else {
                // reach the end, no char anymore
                // ")" not encountered, raise error
                return self.incorrect_function_params_not_close(index_of_left_parenthesis);
            }
        }

        self.final_param_count_check(index_of_left_parenthesis)?;

        Ok(())
    }
}
