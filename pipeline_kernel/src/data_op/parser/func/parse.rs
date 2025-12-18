use crate::FuncParser;
use watchmen_model::StdR;

impl FuncParser<'_> {
    /// now a function name and [(] encountered, so sub path will end with a [)].
    pub fn parse(&mut self, with_context: bool) -> StdR<()> {
        Ok(())
    }
}
