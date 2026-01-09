use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use elf_model::VariablePredefineFunctions;
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

    /// to string
    pub fn to_string(&self) -> String {
        self.full_path[self.start_index..self.end_index]
            .iter()
            .collect()
    }

    pub fn start_index(&self) -> usize {
        self.start_index
    }

    pub fn end_index(&self) -> usize {
        self.end_index
    }
}

/// plain path, a string which is property name
pub struct PlainDataPath {
    pub path: PathStr,
    /// if this path refers to a factor, then should know that the factor is vec (array) or not
    /// otherwise, leave this none when don't know the type
    pub is_vec: Option<bool>,
}

pub enum FuncParamValue {
    Str(String),
    Num(BigDecimal),
    Bool(bool),
    DateTime(NaiveDateTime),
    Date(NaiveDate),
    Time(NaiveTime),
    None,
}

/// value path, a definite value
/// only param of func can be a value path
pub struct FuncParamValuePath {
    pub path: PathStr,
    pub value: FuncParamValue,
}

pub enum FuncDataPathParam {
    Value(FuncParamValuePath),
    Plain(PlainDataPath),
    Func(FuncDataPath),
    Path(DataPath),
}

pub struct FuncDataPath {
    pub path: PathStr,
    pub func: VariablePredefineFunctions,
    pub params: Option<Vec<FuncDataPathParam>>,
}

pub enum DataPathSegment {
    Plain(PlainDataPath),
    Func(FuncDataPath),
}

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
    pub path: PathStr,
    /// at least one segment, which means no [.] included
    pub segments: Vec<DataPathSegment>,
}
