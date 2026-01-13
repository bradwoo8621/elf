use elf_base::StringUtils;
use std::sync::Arc;

pub struct ParserInnerState {
    /// all chars of full path
    all_chars: Arc<Vec<char>>,
    /// current char index of the char which read already
    char_index: usize,
    /// in-memory chars, not consumed yet
    in_memory_chars: String,
}

/// create
impl ParserInnerState {
    /// create new parser inner state, char index is 0, in-memory chars is empty
    pub fn new(all_chars: Arc<Vec<char>>) -> Self {
        ParserInnerState {
            all_chars,
            char_index: 0,
            in_memory_chars: String::new(),
        }
    }

    /// create new parser inner state at current char index of other state, in-memory chars is empty
    pub fn new_at_current_char(other: &Self) -> Self {
        ParserInnerState {
            all_chars: other.all_chars.clone(),
            char_index: other.char_index,
            in_memory_chars: String::new(),
        }
    }

    /// create new parser inner state at current char index of other state,
    /// in-memory chars is copy from other state,
    /// and clear in-memory chars of other state.
    pub fn new_at_current_char_and_copy_in_memory_chars(other: &mut Self) -> Self {
        ParserInnerState {
            all_chars: other.all_chars.clone(),
            char_index: other.char_index,
            in_memory_chars: if other.in_memory_chars_is_empty() {
                String::new()
            } else {
                let chars = other.clone_in_memory_chars();
                other.clear_in_memory_chars();
                chars
            },
        }
    }

    /// create new parser inner state at next char index of other state, in-memory chars is empty
    pub fn new_at_next_char(other: &Self) -> Self {
        ParserInnerState {
            all_chars: other.all_chars.clone(),
            char_index: other.char_index + 1,
            in_memory_chars: String::new(),
        }
    }
}

// for chars
impl ParserInnerState {
    pub fn all_chars(&self) -> &Arc<Vec<char>> {
        &self.all_chars
    }

    /// get full path as string
    pub fn full_path(&self) -> String {
        self.all_chars.iter().collect()
    }

    /// get part of full path as string
    pub fn part_path(&self, start: usize, end: usize) -> String {
        self.all_chars[start..end].iter().collect()
    }

    /// get previous char
    /// char index not change
    pub fn previous_char(&self) -> Option<&char> {
        self.char_at(self.previous_char_index())
    }

    /// get current char
    /// char index not change
    pub fn current_char(&self) -> Option<&char> {
        self.char_at(self.current_char_index() as i64)
    }

    /// get char at given index.
    /// return none if index out of range
    pub fn char_at(&self, char_index: i64) -> Option<&char> {
        if char_index < 0 {
            None
        } else {
            self.all_chars.get(char_index as usize)
        }
    }
}

/// for char index
impl ParserInnerState {
    /// get current char index
    pub fn current_char_index(&self) -> usize {
        self.char_index
    }

    /// get previous char index, will not change current char index
    pub fn previous_char_index(&self) -> i64 {
        self.char_index as i64 - 1
    }

    /// get next char index, will not change current char index
    pub fn next_char_index(&self) -> usize {
        self.char_index + 1
    }

    /// get char index before given chars count, will not change current char index
    pub fn char_index_before_current(&self, chars_count: usize) -> i64 {
        self.char_index as i64 - chars_count as i64
    }

    /// move char index to given index
    pub fn move_char_index_to(&mut self, new_char_index: usize) {
        self.char_index = new_char_index;
    }

    /// move char index to next
    pub fn move_char_index_to_next(&mut self) {
        self.char_index += 1;
    }
}

/// for in-memory chars
impl ParserInnerState {
    /// get in-memory chars
    pub fn in_memory_chars(&self) -> &String {
        &self.in_memory_chars
    }

    /// check the in-memory chars is blank or not
    pub fn in_memory_chars_is_blank(&self) -> bool {
        self.in_memory_chars.is_blank()
    }

    /// check the in-memory chars is empty or not
    pub fn in_memory_chars_is_empty(&self) -> bool {
        self.in_memory_chars.is_empty()
    }

    /// check the in-memory chars is not empty or not
    pub fn in_memory_chars_is_not_empty(&self) -> bool {
        !self.in_memory_chars.is_empty()
    }

    /// get chars count of in-memory chars
    pub fn in_memory_chars_count(&self) -> usize {
        self.in_memory_chars.chars().count()
    }

    pub fn clone_in_memory_chars(&self) -> String {
        self.in_memory_chars.clone()
    }

    pub fn collect_char_into_memory(&mut self, char: char) {
        self.in_memory_chars.push(char)
    }

    // clear in-memory chars
    pub fn clear_in_memory_chars(&mut self) {
        self.in_memory_chars.clear()
    }
}
