use crate::{ParserInnerState, PathStr};

impl ParserInnerState {
    /// create a [PathStr] of [start char index, current char index).
    pub fn create_path_str_exclude_current(&self, start_char_index: usize) -> PathStr {
        PathStr::part_of_chars(
            self.all_chars().clone(),
            start_char_index,
            self.current_char_index(),
        )
    }

    /// create a [PathStr] of in-memory chars, [start char index of in-memory chars, current char index).
    /// make sure the current char index is just after the in-memory chars.
    pub fn create_path_str_of_in_memory_chars(&self) -> PathStr {
        self.create_path_str_exclude_current(
            self.char_index_before_current(self.in_memory_chars_count()) as usize,
        )
    }

    pub fn create_path_str(&self, start_char_index: usize, end_char_index: usize) -> PathStr {
        PathStr::part_of_chars(self.all_chars().clone(), start_char_index, end_char_index)
    }
}
