use crate::{FuncDataPathParam, LiteralConcatFuncParser};

impl LiteralConcatFuncParser<'_> {
    /// append given parameter to the last of params
    pub fn append_param(&mut self, param: FuncDataPathParam) {
        self.params.push(param);
    }
}
