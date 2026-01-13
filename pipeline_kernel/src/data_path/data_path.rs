use crate::{DataPathSegment, PathStr};
use elf_base::DisplayLines;
use std::fmt::{Display, Formatter};

/// data path represents a path to retrieve data
/// could be concatenated by [.], each segment of path can be a plain name or a function
/// e.g. [a.b.c], [a.b.&length], [a.b.&find(c)], [&yearDiff(date1, date2)].
///
/// - All functions are started with [&], and no whitespace in function name.
///   such as [& find], [&fi nd], are illegal.
/// - Some functions are designed not to require context, they are `[&now]`, `[&old]` and `[&nextSeq]`.
/// - Other functions are designed to with/without context. For example, `[a.&length]` is same as `[&length(a)]`.
///   With putting the context object as the first parameter.
/// - If it is a function without parameters, then `[()]` is optional.
/// - It is allowed to use character concatenation with specific syntax to replace the `&concat` function. For example, `[a{a.b}b]`.
///   Here, the `[a]` and `[b]` will be regarded as a string,
///   while `[{a.b}]` will be treated as a standard path, which means getting the value of `[a.b]`.
///   The special feature of this syntax is that it can be recognized as a path.
///   For example, for `[x.a{a.b}.b]`, it will first retrieve the value of `a.b`.
///   Suppose the value is [1], then it appends "a" to it, resulting in "a1". After that, it retrieves the value of "x.a1.b".
///   Therefore, whether it will be recognized as a path depends on whether there are `[.]` before and after.
///   If there are no dots on either side, it will be recognized as a pure string concatenation; otherwise, it will be recognized as a path.
///   There is a special scenario where, when attempting to directly retrieve data from the root data using this syntax,
///   you can use `[&cur.a{a.b}]` or `[&old.a{a.b}]`.
///   Here, `[&cur]` and `[&old]` represent the current data and the previous data respectively.
/// - Provide standard character escaping.
///   If it starts with `[\]` and is immediately followed by one of the characters `[.,(){}&]`, it will be considered an escape.
/// - In functions related to string search and replacement,
///   additional character escaping is provided in the parameters. `[\r\n\t]` will be recognized as line breaks and tabs.
pub struct DataPath {
    path: PathStr,
    /// at least one segment, which means no [.] included
    segments: Vec<DataPathSegment>,
}

impl DataPath {
    pub fn new(path: PathStr, segments: Vec<DataPathSegment>) -> Self {
        Self { path, segments }
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

    pub fn segments(&self) -> &Vec<DataPathSegment> {
        &self.segments
    }
}

impl Display for DataPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.segments.len() == 0 {
            write!(f, "DataPath[{}, segments=[]]", self.path)
        } else {
            let segments_str = self
                .segments
                .iter()
                .map(|s| format!("{}", s))
                .map(DisplayLines::indent)
                .collect::<Vec<String>>()
                .join(",\n");
            write!(
                f,
                "DataPath[{}, segments=[\n{}\n]]",
                self.path, segments_str
            )
        }
    }
}
