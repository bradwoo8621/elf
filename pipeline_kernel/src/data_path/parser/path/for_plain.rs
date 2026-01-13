use crate::{DataPathSegment, DataPathParser, PlainDataPath};
use elf_base::VoidR;

/// consume plain path
impl DataPathParser {
    /// create a plain data path, append to segments. and clear current chars.
    /// blank path is not allowed.
    ///
    /// make sure the current char index is after the in-memory chars
    pub fn consume_in_memory_chars_as_plain_path(
        &mut self,
        move_char_index_to_next: bool,
    ) -> VoidR {
        if self.inner.in_memory_chars_is_blank() {
            return self.incorrect_blank_segment();
        }

        self.append_segment(DataPathSegment::Plain(PlainDataPath {
            path: self.inner.create_path_str_of_in_memory_chars(),
            is_vec: None,
        }));

        self.inner.clear_in_memory_chars();
        if move_char_index_to_next {
            self.inner.move_char_index_to_next()
        }
        Ok(())
    }
}
