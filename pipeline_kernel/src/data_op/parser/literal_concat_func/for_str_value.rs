use crate::{FuncDataPathParam, FuncParamValue, FuncParamValuePath, LiteralConcatFuncParser};
use watchmen_model::StdR;

/// consume str value
impl LiteralConcatFuncParser<'_> {
    /// create a str value, append to params. and clear current chars.
    /// empty is ignored
    /// will not move char index
    pub fn consume_in_memory_chars_as_str(&mut self) -> StdR<()> {
        let inner = &mut self.inner;

        if inner.in_memory_chars_is_empty() {
            return Ok(());
        }

        self.params
            .push(FuncDataPathParam::Value(FuncParamValuePath {
                path: inner.in_memory_chars.clone(),
                value: FuncParamValue::Str(inner.in_memory_chars.clone()),
            }));

        inner.clear_in_memory_chars();

        Ok(())
    }
}
