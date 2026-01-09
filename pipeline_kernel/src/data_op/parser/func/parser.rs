use crate::{FuncDataPathParam, ParserInnerState};
use elf_model::VariablePredefineFunctions;

pub struct FuncParser {
    pub inner: ParserInnerState,
    pub start_char_index_of_func: usize,
    pub func: VariablePredefineFunctions,
    pub params: Vec<FuncDataPathParam>,
    pub with_context: bool,
}
