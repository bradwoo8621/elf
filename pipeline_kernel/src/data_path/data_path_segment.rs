use crate::{FuncDataPath, PathStr, PlainDataPath};
use std::fmt::{Display, Formatter};

pub enum DataPathSegment {
    Plain(PlainDataPath),
    Func(FuncDataPath),
}

impl DataPathSegment {
    pub fn path(&self) -> &PathStr {
        match self {
            Self::Plain(plain_path) => plain_path.path(),
            Self::Func(func_path) => func_path.path(),
        }
    }

    pub fn this_path(&self) -> String {
        self.path().this()
    }

    pub fn full_path(&self) -> String {
        self.path().full()
    }

    /// return position is included
    pub fn start_at(&self) -> usize {
        self.path().start_index()
    }

    /// return position is excluded
    pub fn end_at(&self) -> usize {
        self.path().end_index()
    }
}

impl Display for DataPathSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plain(plain) => write!(f, "{}", plain),
            Self::Func(func) => write!(f, "{}", func),
        }
    }
}
