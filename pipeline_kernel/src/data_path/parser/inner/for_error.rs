use crate::{ParserInnerState, PipelineKernelErrorCode};
use elf_base::{ErrorCode, StdErrCode, StdR};

/// report error
impl ParserInnerState {
    #[track_caller]
    pub fn error<S, R>(&self, msg: S) -> StdR<R>
    where
        S: Into<String>,
    {
        PipelineKernelErrorCode::IncorrectDataPath.msg(msg)
    }

    #[track_caller]
    pub fn incorrect_char_at_previous_index<R>(&self, char: &char) -> StdR<R> {
        self.error(format!(
            "Incorrect data path[{}], caused by incorrect {} at index[{}].",
            self.full_path(),
            char,
            self.previous_char_index()
        ))
    }

    #[track_caller]
    fn incorrect_char_at_index<R>(&self, reason: &str) -> StdR<R> {
        self.error(format!(
            "Incorrect data path[{}], caused by incorrect {} at index[{}].",
            self.full_path(),
            reason,
            self.current_char_index()
        ))
    }

    #[track_caller]
    pub fn incorrect_dot<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("dot")
    }

    #[track_caller]
    pub fn incorrect_comma<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("comma")
    }

    #[track_caller]
    pub fn incorrect_left_parenthesis<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("left parenthesis")
    }

    #[track_caller]
    pub fn incorrect_right_parenthesis<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("right parenthesis")
    }

    #[track_caller]
    pub fn incorrect_left_brace<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("left brace")
    }

    #[track_caller]
    pub fn incorrect_right_brace<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("right brace")
    }

    #[track_caller]
    pub fn incorrect_ampersand<R>(&self) -> StdR<R> {
        self.incorrect_char_at_index("ampersand")
    }

    #[track_caller]
    pub fn unknown_error<R>(&self) -> StdR<R> {
        StdErrCode::Unknown.msg("Unknown error occurred during data path parsing.")
    }
}
