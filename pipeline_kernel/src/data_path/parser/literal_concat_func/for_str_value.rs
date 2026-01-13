use crate::{FuncDataPathParam, FuncParamValue, FuncParamValuePath, LiteralConcatFuncParser};
use elf_base::VoidR;

/// consume str value
impl LiteralConcatFuncParser {
    /// create a str value, append to params. and clear current chars.
    /// empty is ignored
    /// will not move char index
    pub fn consume_in_memory_chars_as_str(&mut self) -> VoidR {
        if self.inner.in_memory_chars_is_empty() {
            return Ok(());
        }

        self.params
            .push(FuncDataPathParam::Value(FuncParamValuePath::new(
                self.inner.create_path_str_of_in_memory_chars(),
                FuncParamValue::Str(self.inner.clone_in_memory_chars()),
            )));

        self.inner.clear_in_memory_chars();

        Ok(())
    }
}
