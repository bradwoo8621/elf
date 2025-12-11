use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use watchmen_model::VariablePredefineFunctions;

pub struct PlainDataPath {
    pub path: String,
    /// if this path refers to a factor, then should know that the factor is vec (array) or not
    /// otherwise, leave this none when don't know the type
    pub is_vec: Option<bool>,
}

pub enum FuncDataPathParamPart {
    Str(String),
    Num(BigDecimal),
    Bool(bool),
    DateTime(NaiveDateTime),
    Date(NaiveDate),
    Time(NaiveTime),
    Variable(ParsedDataPath),
}

pub struct FuncDataPathParam {
    pub path: String,
    pub parts: Vec<FuncDataPathParamPart>,
}

pub struct FuncDataPath {
    pub path: String,
    pub func: VariablePredefineFunctions,
    pub params: Option<Vec<FuncDataPathParam>>,
}

pub enum DataPathSegment {
    Plain(PlainDataPath),
    Func(FuncDataPath),
}

pub struct ParsedDataPath {
    pub path: String,
    /// at least one segment, which means no [.] included
    pub segments: Vec<DataPathSegment>,
}

pub enum DataPath {
    Simple(ParsedDataPath),
    Complex(ParsedDataPath),
}
