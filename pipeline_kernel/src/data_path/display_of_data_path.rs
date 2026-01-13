use crate::{
    DataPath, DataPathSegment, FuncDataPath, FuncDataPathParam, FuncParamValue, FuncParamValuePath,
    PathStr, PlainDataPath,
};
use elf_base::DisplayLines;
use std::fmt::{Display, Formatter, Result};

impl Display for PathStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "PathStr[text={}, range=[{}, {})]",
            self.to_string(),
            self.start_index(),
            self.end_index()
        )
    }
}

impl Display for PlainDataPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "PlainDataPath[{}, is_vec={}]",
            self.path,
            self.is_vec.map_or("none".to_string(), |v| v.to_string())
        )
    }
}

impl Display for FuncParamValuePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

impl Display for FuncDataPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut params_str = "none".to_string();
        if let Some(params) = &self.params {
            if params.len() != 0 {
                params_str = params
                    .iter()
                    .map(|p| match p {
                        FuncDataPathParam::Value(v) => format!("{}", v),
                        FuncDataPathParam::Plain(p) => format!("{}", p),
                        FuncDataPathParam::Func(func) => format!("{}", func),
                        FuncDataPathParam::Path(path) => format!("{}", path),
                    })
                    .map(DisplayLines::indent)
                    .collect::<Vec<String>>()
                    .join(",\n");
                params_str = format!("[\n{}\n]", params_str);
            }
        }
        write!(
            f,
            "FuncDataPath[{}, func={}, params={}]",
            self.path, self.func, params_str
        )
    }
}

impl Display for DataPathSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Plain(plain) => write!(f, "{}", plain),
            Self::Func(func) => write!(f, "{}", func),
        }
    }
}

impl Display for DataPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.segments.len() == 0 {
            write!(f, "DataPath[text={}, segments=[]]", self.path.to_string())
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
