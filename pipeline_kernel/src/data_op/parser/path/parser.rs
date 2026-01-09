use crate::{DataPathSegment, FuncDataPath, ParserInnerState};
use elf_model::VariablePredefineFunctions;
use std::sync::Arc;

pub struct PathParser {
    pub inner: ParserInnerState,
    pub segments: Vec<DataPathSegment>,
}

impl PathParser {
    pub fn by_path(all_chars: Arc<Vec<char>>) -> Self {
        PathParser {
            inner: ParserInnerState::new(all_chars),
            segments: vec![],
        }
    }
}

/// segments operations
impl PathParser {
    /// get last segment if it is a concat function
    /// raise error by given [not_found] when,
    /// - last segment is not a concat function,
    /// - segments is empty
    pub fn pop_last_concat_function(&mut self) -> Option<FuncDataPath> {
        if let Some(last_segment) = self.segments.last() {
            match last_segment {
                DataPathSegment::Func(data_path) => match data_path.func {
                    VariablePredefineFunctions::Concat => {
                        if let Some(segment) = self.segments.pop() {
                            match segment {
                                DataPathSegment::Func(data_path) => Some(data_path),
                                // never happen, checked already
                                _ => None,
                            }
                        } else {
                            // never happen, checked already
                            None
                        }
                    }
                    _ => None,
                },
                // last segment cannot be a plain path,
                // never happen since char before is "}", never be a plain path
                _ => None,
            }
        } else {
            // segments is empty
            None
        }
    }

    /// check there is segment existing or not
    pub fn has_segment(&self) -> bool {
        !self.segments.is_empty()
    }

    /// append given segment to the last of segments
    pub fn append_segment(&mut self, segment: DataPathSegment) {
        self.segments.push(segment);
    }
}
