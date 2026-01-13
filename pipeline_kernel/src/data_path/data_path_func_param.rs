use crate::{DataPath, FuncDataPath, PathStr, PlainDataPath};
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::fmt::{Display, Formatter};

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
    path: PathStr,
    value: FuncParamValue,
}

impl FuncParamValuePath {
    pub fn new(path: PathStr, value: FuncParamValue) -> Self {
        Self { path, value }
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

    pub fn value(&self) -> &FuncParamValue {
        &self.value
    }
}

impl Display for FuncParamValuePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value_str = match &self.value {
            FuncParamValue::Str(s) => format!("Str({})", s),
            FuncParamValue::Num(n) => format!("Num({})", n),
            FuncParamValue::Bool(b) => format!("Bool({})", b),
            FuncParamValue::DateTime(dt) => format!("DateTime({})", dt),
            FuncParamValue::Date(d) => format!("Date({})", d),
            FuncParamValue::Time(t) => format!("Time({})", t),
            FuncParamValue::None => "none".to_string(),
        };
        write!(f, "FuncParamValuePath[{}, value={}]", self.path, value_str)
    }
}

pub enum FuncDataPathParam {
    Value(FuncParamValuePath),
    Plain(PlainDataPath),
    Func(FuncDataPath),
    Path(DataPath),
}
