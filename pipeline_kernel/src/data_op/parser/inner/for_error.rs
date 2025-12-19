use crate::{ParserInnerState, PipelineKernelErrorCode};
use watchmen_model::{StdErrCode, StdErrorCode, StdR, VariablePredefineFunctions};

/// report error
impl ParserInnerState<'_> {
    fn error<S, R>(&self, msg: S) -> StdR<R>
    where
        S: Into<String>,
    {
        PipelineKernelErrorCode::IncorrectDataPath.msg(msg)
    }

    pub fn incorrect_empty_path<R>(&self) -> StdR<R> {
        self.error("Incorrect data path, caused by not content determined.")
    }

    pub fn incorrect_char_at_previous_index<R>(&self, char: &char) -> StdR<R> {
        self.error(format!(
            "Incorrect data path[{}], caused by incorrect {} at index[{}].",
            self.full_path,
            char,
            self.char_index - 1
        ))
    }

    fn incorrect_char_at_index<R>(&self, reason: &str) -> StdR<R> {
        self.error(format!(
            "Incorrect data path[{}], caused by incorrect {} at index[{}].",
            self.full_path, reason, self.char_index
        ))
    }

    pub fn incorrect_dot<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("dot")
    }

    pub fn incorrect_comma<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("comma")
    }

    pub fn incorrect_left_parenthesis<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("left parenthesis")
    }

    pub fn incorrect_right_parenthesis<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("right parenthesis")
    }

    pub fn incorrect_left_brace<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("left brace")
    }

    pub fn incorrect_right_brace<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("right brace")
    }

    pub fn incorrect_ampersand<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("ampersand")
    }

    /// start is included, end is excluded
    pub fn incorrect_blank_segment<R>(
        &self,
        start_char_index: usize,
        end_char_index: usize,
    ) -> StdR<R> {
        self.error(format!(
            "Incorrect data path[{}], caused by blank segment at index[{}, {}].",
            self.full_path, start_char_index, end_char_index
        ))
    }

    pub fn incorrect_wrapped_path<R>(&self, start_char_index: usize) -> StdR<R> {
        self.error(format!(
            "Incorrect data path[{}], caused by the closing \"}}\" is not matched, the opening \"{{\" is at index [{}].",
            self.full_path, start_char_index
        ))
    }

    pub fn incorrect_function_name_char<R>(&self, char: char) -> StdR<R> {
        self.error(format!(
            "Incorrect data path[{}], caused by disallowed char[{}] in function name at index[{}].",
            self.full_path, char, self.char_index
        ))
    }

    pub fn incorrect_empty_function_name<R>(&self, start_char_index: usize) -> StdR<R> {
        self.error(format!(
            "Incorrect data path[{}], caused by empty function name at index[{}].",
            self.full_path, start_char_index
        ))
    }

    pub fn incorrect_function_name<R>(
        &self,
        start_char_index: usize,
        end_char_index: usize,
    ) -> StdR<R> {
        self.error(format!(
            "Incorrect data path[{}], caused by unrecognized function name[{}] at index[{}, {}].",
            self.full_path,
            self.full_path[start_char_index..end_char_index].to_string(),
            start_char_index,
            end_char_index
        ))
    }

    pub fn incorrect_function_name_contains_whitespace<R>(
        &self,
        start_char_index: usize,
        end_char_index: usize,
    ) -> StdR<R> {
        self.error(format!(
            "Incorrect data path[{}], caused by function name[{}] contains whitespace(s) at index[{}, {}].",
            self.full_path,
            self.full_path[start_char_index..end_char_index].to_string(),
            start_char_index,
            end_char_index
        ))
    }

    pub fn incorrect_function_no_param<R>(
        &self,
        start_char_index: usize,
        end_char_index: usize,
    ) -> StdR<R> {
        self.error(format!(
            "Incorrect data path[{}], caused by function[{}] must have parameter(s) at index[{}, {}].",
            self.full_path,
            self.full_path[start_char_index..end_char_index].to_string(),
            start_char_index,
            end_char_index
        ))
    }

    pub fn incorrect_function_has_context<R>(
        &self,
        start_char_index: usize,
        end_char_index: usize,
    ) -> StdR<R> {
        self.error(format!(
            "Incorrect data path[{}], caused by function[{}] cannot have context at index[{}, {}].",
            self.full_path,
            self.full_path[start_char_index..end_char_index].to_string(),
            start_char_index,
            end_char_index
        ))
    }

    pub fn incorrect_function_params<R>(&self, start_char_index: usize) -> StdR<R> {
        self.error(format!(
            "Incorrect data path[{}], caused by the closing \")\" is not matched, the opening \"(\" is at index [{}].",
            self.full_path, start_char_index
        ))
    }

    pub fn incorrect_function_param_over_max_count<R>(
        &self,
        func: &VariablePredefineFunctions,
        start_char_index: usize,
        end_char_index: usize,
        max_count: usize,
    ) -> StdR<R> {
        self.error(format!(
            "Incorrect data path[{}], caused by function[{}] can accept a maximum of {} parameters at index[{}, {}].",
            self.full_path,
            func.to_string(),
            max_count,
            start_char_index,
            end_char_index
        ))
    }

    pub fn unknown_error<R>(&self) -> StdR<R> {
        StdErrCode::Unknown.msg("Unknown error occurred during data path parsing.")
    }
}
