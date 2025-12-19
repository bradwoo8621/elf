use crate::FuncParser;
use watchmen_model::StdR;

impl FuncParser<'_> {
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
    ) -> StdR<()> {
        let max_param_count = self.func.max_param_count();
        if max_param_count.is_none() {
            return Ok(());
        }

        if self.with_context && !self.func.require_context() {
            // basically, never happen
            return self.inner.incorrect_function_has_context(
                self.start_char_index_of_func,
                self.inner.char_index,
            );
        }

        let max_param_count = max_param_count.unwrap();
        let parsed_param_count = self.params.len();
        let param_on_air_count = (param_index == parsed_param_count)
            .then_some(1)
            .unwrap_or(0);
        let context_param_count = (!self.with_context && self.func.require_context())
            .then_some(1)
            .unwrap_or(0);
        let more_param_count = one_more_param_booked.then_some(1).unwrap_or(0);
        let at_least_param_count =
            parsed_param_count + param_on_air_count + more_param_count - context_param_count;
        if at_least_param_count > max_param_count {
            return self.inner.incorrect_function_param_over_max_count(
                &self.func,
                index_of_left_parenthesis,
                self.inner.char_index,
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
    ) -> StdR<()> {
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
    ) -> StdR<()> {
        self.check_param_count(index_of_left_parenthesis, param_index, true)
    }
}
