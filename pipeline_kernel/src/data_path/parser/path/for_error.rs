use crate::DataPathParser;
use elf_base::StdR;
use elf_model::VariablePredefineFunctions;

impl DataPathParser {
    /// path is empty
    #[track_caller]
    pub fn incorrect_empty_path<R>(&self) -> StdR<R> {
        self.inner
            .error("Incorrect data path, caused by not content determined.")
    }

    /// report error at [index of blank string start, current char index)
    #[track_caller]
    pub fn incorrect_blank_segment<R>(&self) -> StdR<R> {
        self.inner.error(format!(
            "Incorrect data path[{}], caused by blank segment at index[{}, {}].",
            self.inner.full_path(),
            self.inner
                .char_index_before_current(self.inner.in_memory_chars_count()),
            self.inner.current_char_index(),
        ))
    }

    /// report error at [index of left brace]
    #[track_caller]
    pub fn incorrect_wrapped_path<R>(&self, index_of_left_brace: usize) -> StdR<R> {
        self.inner.error(format!(
            "Incorrect data path[{}], caused by the closing \"}}\" is not matched, the opening \"{{\" is at index [{}].",
            self.inner.full_path(), index_of_left_brace
        ))
    }

    /// report error at [current char index]
    #[track_caller]
    pub fn incorrect_function_name_char<R>(&self, char: char) -> StdR<R> {
        self.inner.error(format!(
            "Incorrect data path[{}], caused by disallowed char[{}] in function name at index[{}].",
            self.inner.full_path(),
            char,
            self.inner.current_char_index()
        ))
    }

    /// report error at [current char index - 1]
    #[track_caller]
    pub fn incorrect_empty_function_name<R>(&self) -> StdR<R> {
        self.inner.error(format!(
            "Incorrect data path[{}], caused by empty function name at index[{}].",
            self.inner.full_path(),
            self.inner.previous_char_index()
        ))
    }

    /// report error at [current char index - in memory chars count, current char index)
    #[track_caller]
    pub fn incorrect_function_name<R>(&self, in_memory_chars_count: usize) -> StdR<R> {
        let end_char_index = self.inner.current_char_index();
        let start_char_index = self.inner.char_index_before_current(in_memory_chars_count) as usize;
        self.inner.error(format!(
            "Incorrect data path[{}], caused by unrecognized function name[{}] at index[{}, {}].",
            self.inner.full_path(),
            self.inner.part_path(start_char_index, end_char_index),
            start_char_index,
            end_char_index
        ))
    }

    /// report error at [index of ampersand, current char index + 1)
    #[track_caller]
    pub fn incorrect_function_name_contains_whitespace<R>(
        &self,
        index_of_ampersand: usize,
    ) -> StdR<R> {
        let end_char_index = self.inner.next_char_index();
        self.inner.error(format!(
            "Incorrect data path[{}], caused by function name[{}] contains whitespace(s) at index[{}, {}].",
            self.inner.full_path(),
            self.inner.part_path(index_of_ampersand,end_char_index),
            index_of_ampersand,
            end_char_index
        ))
    }

    /// report error at [index of ampersand, current char index)
    #[track_caller]
    pub fn incorrect_function_no_param<R>(&self, func: &VariablePredefineFunctions) -> StdR<R> {
        let end_char_index = self.inner.current_char_index();
        let start_char_index =
            self.inner
                .char_index_before_current(func.to_string().chars().count()) as usize;
        self.inner.error(format!(
            "Incorrect data path[{}], caused by function[{}] must have parameter(s) at index[{}, {}].",
            self.inner.full_path(),
            self.inner.part_path(start_char_index,end_char_index),
            start_char_index,
            end_char_index
        ))
    }

    /// report error at [index of ampersand, current char index)
    #[track_caller]
    pub fn incorrect_function_has_context<R>(&self, index_of_ampersand: usize) -> StdR<R> {
        let end_char_index = self.inner.current_char_index();
        self.inner.error(format!(
            "Incorrect data path[{}], caused by function[{}] cannot have context at index[{}, {}].",
            self.inner.full_path(),
            self.inner.part_path(index_of_ampersand, end_char_index),
            index_of_ampersand,
            end_char_index
        ))
    }

    /// report error at [index of param start]
    #[track_caller]
    pub fn incorrect_function_param_not_close<R>(&self, param_start_char_index: usize) -> StdR<R> {
        self.inner.error(format!(
            "Incorrect data path[{}], caused by the closing \",\" or \")\" is not matched, parameter starts at index [{}].",
            self.inner.full_path(), param_start_char_index
        ))
    }
}
