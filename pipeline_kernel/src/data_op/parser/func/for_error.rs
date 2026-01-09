use crate::FuncParser;
use elf_base::StdR;

impl FuncParser {
    /// report error at [index of ampersand, current char index)
    pub fn incorrect_function_has_context<R>(&self) -> StdR<R> {
        let start_char_index = self.start_char_index_of_func;
        let end_char_index = self.inner.current_char_index();
        self.inner.error(format!(
            "Incorrect data path[{}], caused by function[{}] cannot have context at index[{}, {}].",
            self.inner.full_path(),
            self.inner.part_path(start_char_index, end_char_index),
            start_char_index,
            end_char_index
        ))
    }

    /// report error at [index of ampersand, current char index)
    pub fn incorrect_function_has_no_context<R>(&self) -> StdR<R> {
        let start_char_index = self.start_char_index_of_func;
        let end_char_index = self.inner.current_char_index();
        self.inner.error(format!(
            "Incorrect data path[{}], caused by function[{}] must have context at index[{}, {}].",
            self.inner.full_path(),
            self.inner.part_path(start_char_index, end_char_index),
            start_char_index,
            end_char_index
        ))
    }

    /// report error at [index of left parenthesis]
    pub fn incorrect_function_params_not_close<R>(
        &self,
        index_of_left_parenthesis: usize,
    ) -> StdR<R> {
        self.inner.error(format!(
            "Incorrect data path[{}], caused by the closing \")\" is not matched, the opening \"(\" is at index [{}].",
            self.inner.full_path(), index_of_left_parenthesis
        ))
    }

    /// report error at [index of left parenthesis, current char index)
    pub fn incorrect_function_param_over_max_count<R>(
        &self,
        index_of_left_parenthesis: usize,
        max_count: usize,
    ) -> StdR<R> {
        self.inner.error(format!(
            "Incorrect data path[{}], caused by function[{}] can accept a maximum of {} parameters at index[{}, {}].",
            self.inner.full_path(),
            self.func.to_string(),
            max_count,
            index_of_left_parenthesis,
            self.inner.current_char_index()
        ))
    }

    /// report error at [index of left parenthesis, current char index)
    pub fn incorrect_function_param_below_min_count<R>(
        &self,
        index_of_left_parenthesis: usize,
        min_count: usize,
    ) -> StdR<R> {
        self.inner.error(format!(
            "Incorrect data path[{}], caused by function[{}] can accept a minimum of {} parameters at index[{}, {}].",
            self.inner.full_path(),
            self.func.to_string(),
            min_count,
            index_of_left_parenthesis,
            self.inner.current_char_index()
        ))
    }

    /// report error at [current char index - in memory chars count, current char index)
    pub fn incorrect_function_param_tailing_whitespaces<R>(
        &self,
        in_memory_chars_count: usize,
    ) -> StdR<R> {
        self.inner.error(format!(
            "Incorrect data path[{}], caused by function[{}] parameter has unexpected tailing whitespaces at index[{}, {}].",
            self.inner.full_path(),
            self.func.to_string(),
            self.inner.char_index_before_current(in_memory_chars_count),
            self.inner.current_char_index()
        ))
    }

    /// report error at [current char index - in memory chars count, current char index)
    pub fn incorrect_function_invalid_context<R>(&self, chars_count: usize) -> StdR<R> {
        self.inner.error(format!(
            "Incorrect data path[{}], caused by context of function[{}] is invalid at index[{}, {}].",
            self.inner.full_path(),
            self.func.to_string(),
            self.inner.char_index_before_current(chars_count),
            self.inner.current_char_index()
        ))
    }
}
