use crate::{FuncDataPathParam, ParserInnerState};
use elf_model::VariablePredefineFunctions;

/// data path, function parser
pub struct DataPathFuncParser {
    pub inner: ParserInnerState,
    pub start_char_index_of_func: usize,
    pub func: VariablePredefineFunctions,
    pub params: Vec<FuncDataPathParam>,
    pub with_context: bool,
}
