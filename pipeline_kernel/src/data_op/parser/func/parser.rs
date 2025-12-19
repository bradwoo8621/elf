use crate::{FuncDataPathParam, ParserInnerState};
use watchmen_model::VariablePredefineFunctions;

pub struct FuncParser<'a> {
    pub inner: ParserInnerState<'a>,
    pub start_char_index_of_func: usize,
    pub func: VariablePredefineFunctions,
    pub params: Vec<FuncDataPathParam>,
    pub with_context: bool,
}
