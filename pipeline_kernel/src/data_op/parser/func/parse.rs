use crate::FuncParser;
use watchmen_model::StdR;

impl FuncParser<'_> {
    fn parse_param(&mut self) -> StdR<()> {
        // TODO
        Ok(())
    }

    fn end_param(&mut self) -> StdR<()> {
        // TODO
        // move to next char, skip the ")" or ","
        self.inner.move_char_index_to_next();
        Ok(())
    }

    fn end_param_before_right_parenthesis(
        &mut self,
        index_of_left_parenthesis: usize,
        param_index: usize,
    ) -> StdR<()> {
        self.check_param_count_before_right_parenthesis(index_of_left_parenthesis, param_index)?;
        self.end_param()
    }

    /// it is not the last parameter of function, there is one more parameter will be parsed after comma.
    fn end_param_before_comma(
        &mut self,
        index_of_left_parenthesis: usize,
        param_index: usize,
    ) -> StdR<()> {
        self.check_param_count_before_comma(index_of_left_parenthesis, param_index)?;
        self.end_param()
    }

    fn finalize_content(&mut self) -> StdR<()> {
        // TODO
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
    pub fn parse(&mut self) -> StdR<()> {
        let index_of_left_parenthesis = self.inner.char_index - 1;

        // total parsed parameter count
        let mut param_index: usize = 0;
        let mut whitespace_met = true;
        loop {
            if let Some(char) = self.inner.current_char() {
                match char {
                    '&' => {
                        if whitespace_met {
                            self.inner.incorrect_ampersand()?;
                        } else {
                            // ignore the whitespaces before function
                            self.inner.clear_in_memory_chars();
                            self.parse_param()?;
                            whitespace_met = false;
                        }
                    }
                    '(' => self.inner.incorrect_left_parenthesis()?,
                    // end of parameters
                    ')' => {
                        self.end_param_before_right_parenthesis(
                            index_of_left_parenthesis,
                            param_index,
                        )?;
                        param_index += 1;
                        break;
                    }
                    // end of parameter
                    ',' => {
                        self.end_param_before_comma(index_of_left_parenthesis, param_index)?;
                        param_index += 1;
                    }
                    // \s, ignore.
                    // in python version, whitespaces after function name is allowed, compatible logic here
                    ' ' | '\t' | '\r' | '\n' | '\x0C' | '\x0B' => {
                        whitespace_met = true;
                        self.inner
                            .consume_char_into_memory_and_move_char_index_to_next(*char)
                    }
                    // other char
                    _ => {
                        self.parse_param()?;
                        whitespace_met = false;
                    }
                };
            } else {
                // reach the end, no char anymore
                // ")" not encountered, raise error
                return self
                    .inner
                    .incorrect_function_params(index_of_left_parenthesis);
            }
        }

        // TODO finalize parsed function
        self.finalize_content()?;

        Ok(())
    }
}
