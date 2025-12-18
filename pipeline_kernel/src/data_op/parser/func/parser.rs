use crate::{FuncDataPathParam, ParserInnerState};
use watchmen_model::VariablePredefineFunctions;

pub struct FuncParser<'a> {
    pub inner: ParserInnerState<'a>,
    pub func: VariablePredefineFunctions,
    pub params: Vec<FuncDataPathParam>,
}
