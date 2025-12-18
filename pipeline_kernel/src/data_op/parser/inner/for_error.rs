use crate::{ParserInnerState, PipelineKernelErrorCode};
use watchmen_model::{StdErrCode, StdErrorCode, StdR};

/// report error
impl ParserInnerState<'_> {
    pub fn incorrect_empty_path<R>(&self) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath
            .msg("Incorrect data path, caused by not content determined.")
    }

    pub fn incorrect_char_at_previous_index<R>(&self, char: &char) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
            "Incorrect data path[{}], caused by incorrect {} at index[{}].",
            self.full_path, char, self.char_index - 1
        ))
    }

    fn incorrect_char_at_index<R>(&self, reason: &str) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
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
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
            "Incorrect data path[{}], caused by blank segment at index[{}, {}].",
            self.full_path, start_char_index, end_char_index
        ))
    }

    pub fn incorrect_wrapped_path<R>(&self, start_char_index: usize) -> StdR<R> {
        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
            "Incorrect data path[{}], caused by the closing \"}}\" is not matched, the opening \"{{\" is at index [{}].",
            self.full_path, start_char_index
        ))
    }

    pub fn unknown_error<R>(&self) -> StdR<R> {
        StdErrCode::Unknown.msg("Unknown error occurred during data path parsing.")
    }
}
