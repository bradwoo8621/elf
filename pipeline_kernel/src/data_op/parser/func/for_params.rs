use crate::FuncParser;
use elf_base::VoidR;

impl FuncParser {
    /// get real param index in function definition
    ///
    /// never call this function when
    /// - the given param index is 0,
    /// - and the function requires context,
    /// - and the current parsing is without context.
    /// in above case, real param index will be -1, and for return type usize, which is invalid.
    /// but this function will return 0 in this case to avoid panic,
    /// and the caller should make sure not to call this function in above case.
    pub fn real_param_index(&self, param_index: usize) -> usize {
        if self.with_context {
            // with context, param index is real param index
            param_index
        } else if self.func.require_context() {
            // no context, but require context, which means the first parameter is context
            // so real param index is param index - 1
            if param_index == 0 { 0 } else { param_index - 1 }
        } else {
            // no context and not require it, so real param index is param index
            param_index
        }
    }

    /// - [param_index]: starts from 0,
    /// - [one_more_param_booked]:
    ///   - if [,] detected,
    ///     it means that there is still one parameter to be parsed in the future, therefore pass [true],
    ///   - if [)] detected, this is the last parameter, therefore pass [false].
    fn check_param_count(
        &mut self,
        index_of_left_parenthesis: usize,
        param_index: usize,
        one_more_param_booked: bool,
    ) -> VoidR {
        let max_param_count = self.func.max_param_count();
        if max_param_count.is_none() {
            // no parameter limit, pass
            return Ok(());
        }

        if self.with_context && !self.func.require_context() {
            // basically, never happen, already checked
            return self.incorrect_function_has_context();
        }

        let max_param_count = max_param_count.unwrap();
        // parameter parsed so far
        let parsed_param_count = self.params.len();
        // when the current parameter is not parsed yet.
        // basically, no matter there is in-memory chars, or just empty,
        // it can be treated as blank/empty string or none, unless function does not allow it.
        // which means one more parameter to be counted.
        // but still there is a special case, which is
        // - param index is 0,
        // - function does not allow context or already has context,
        // - function's max param count is 0,
        // in this case, the whitespaces in memory will be ignored.
        // that is saying, in this case, parameter on air count is 0.
        let mut param_on_air_count = 0;
        if param_index >= parsed_param_count {
            // typically, there is one parameter on air
            param_on_air_count = 1;
            if param_index == 0 && max_param_count == 0 && !one_more_param_booked {
                // current is the first parameter, and max param count is 0,
                // and there is a right parenthesis (not a comma) detected,
                if !self.func.require_context() || self.with_context {
                    // - context disallowed,
                    // - or context allowed, but already provided,
                    // in this case, the whitespaces in memory will be ignored.
                    param_on_air_count = 0;
                }
            }
        }
        // when the first parameter is context, not counted in user parameters
        let context_param_count = (!self.with_context && self.func.require_context())
            .then_some(1)
            .unwrap_or(0);
        // when [,] detected, one more parameter to be parsed in the future
        let more_param_count = one_more_param_booked.then_some(1).unwrap_or(0);
        let at_least_param_count =
            parsed_param_count + param_on_air_count + more_param_count - context_param_count;
        if at_least_param_count > max_param_count {
            return self.incorrect_function_param_over_max_count(
                index_of_left_parenthesis,
                max_param_count,
            );
        }

        Ok(())
    }

    /// check param count before a [,] detected.
    /// - max param count is none, no limit, pass,
    /// - if current is with context, and not requires context, raise error, not allowed.
    /// - otherwise,
    ///   [at least parameters count for now] =
    ///   - [parsed count]
    ///   - [+ param index == parsed count ? 1 : 0]: param index starts from 0,
    ///     when param index < parsed count, means this parameter not parsed yet,
    ///     it might be in-memory or just no content (e.g. [(,] or [,,])
    ///   - [- not with context && requires context ? 1: 0]: first parameter, context.
    ///
    ///   raise error when [at least parameters count > max param count]
    pub fn check_param_count_before_right_parenthesis(
        &mut self,
        index_of_left_parenthesis: usize,
        param_index: usize,
    ) -> VoidR {
        self.check_param_count(index_of_left_parenthesis, param_index, false)
    }

    /// check param count before a [,] detected.
    /// - max param count is none, no limit, pass,
    /// - if current is with context, and not requires context, raise error, not allowed.
    /// - otherwise,
    ///   [at least parameters count for now] =
    ///   - [parsed count]
    ///   - [+ param index == parsed count ? 1 : 0]: param index starts from 0,
    ///     when param index < parsed count, means this parameter not parsed yet,
    ///     it might be in-memory or just no content (e.g. [(,] or [,,])
    ///   - [+ 1]: one after comma, will be parsed at next round,
    ///   - [- not with context && requires context ? 1: 0]: first parameter, context.
    ///
    ///   raise error when [at least parameters count > max param count]
    pub fn check_param_count_before_comma(
        &mut self,
        index_of_left_parenthesis: usize,
        param_index: usize,
    ) -> VoidR {
        self.check_param_count(index_of_left_parenthesis, param_index, true)
    }
}
