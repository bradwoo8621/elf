use std::fmt::{Display, Formatter};
use std::sync::Arc;

/// path string with start and end index in the full string
pub struct PathStr {
    full_path: Arc<Vec<char>>,
    start_index: usize,
    end_index: usize,
}

impl PathStr {
    /// init with given full path, start index is 0, end index is length of full path
    pub fn of_str(full_path: &str) -> Self {
        let path: Vec<char> = full_path.chars().collect();
        let end_index = path.len();

        PathStr {
            full_path: Arc::new(path),
            start_index: 0,
            end_index,
        }
    }

    /// init with given full path (chars), start index is 0, end index is length of full path
    pub fn of_chars(full_path: Arc<Vec<char>>) -> Self {
        let end_index = full_path.len();

        PathStr {
            full_path,
            start_index: 0,
            end_index,
        }
    }

    /// init with given full path (chars), start index and end index
    pub fn part_of_chars(full_path: Arc<Vec<char>>, start_index: usize, end_index: usize) -> Self {
        PathStr {
            full_path,
            start_index,
            end_index,
        }
    }

    /// get string path
    pub fn this(&self) -> String {
        self.full_path[self.start_index..self.end_index]
            .iter()
            .collect()
    }

    // get full string path
    pub fn full(&self) -> String {
        self.full_path.iter().collect()
    }

    pub fn start_index(&self) -> usize {
        self.start_index
    }

    pub fn end_index(&self) -> usize {
        self.end_index
    }
}

impl Display for PathStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PathStr[text={}, range=[{}, {})]",
            self.this(),
            self.start_index(),
            self.end_index()
        )
    }
}
