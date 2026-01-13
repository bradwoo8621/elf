use crate::PathStr;
use std::fmt::{Display, Formatter};

/// plain path, a string which is property name
pub struct PlainDataPath {
    path: PathStr,
    /// if this path refers to a factor, then should know that the factor is vec (array) or not
    /// otherwise, leave this none when don't know the type
    is_vec: Option<bool>,
}

impl PlainDataPath {
    pub fn new(path: PathStr, is_vec: Option<bool>) -> Self {
        Self { path, is_vec }
    }

    pub fn path(&self) -> &PathStr {
        &self.path
    }

    pub fn this_path(&self) -> String {
        self.path.this()
    }

    pub fn full_path(&self) -> String {
        self.path.full()
    }

    /// return position is included
    pub fn start_at(&self) -> usize {
        self.path.start_index()
    }

    /// return position is excluded
    pub fn end_at(&self) -> usize {
        self.path.end_index()
    }

    pub fn is_vec(&self) -> Option<bool> {
        self.is_vec
    }
}

impl Display for PlainDataPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PlainDataPath[{}, is_vec={}]",
            self.path,
            self.is_vec.map_or("none".to_string(), |v| v.to_string())
        )
    }
}
