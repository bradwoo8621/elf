use crate::{FuncDataPathParam, PathStr};
use elf_base::DisplayLines;
use elf_model::VariablePredefineFunctions;
use std::fmt::{Display, Formatter};

pub struct FuncDataPath {
    path: PathStr,
    func: VariablePredefineFunctions,
    params: Option<Vec<FuncDataPathParam>>,
}

impl FuncDataPath {
    pub fn new(
        path: PathStr,
        func: VariablePredefineFunctions,
        params: Option<Vec<FuncDataPathParam>>,
    ) -> Self {
        Self { path, func, params }
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

    pub fn func(&self) -> &VariablePredefineFunctions {
        &self.func
    }

    pub fn params(&self) -> &Option<Vec<FuncDataPathParam>> {
        &self.params
    }

    /// take params from path, leaving [none] instead
    /// if origin params is [none], return empty vec.
    pub fn take_params(&mut self) -> Vec<FuncDataPathParam> {
        let params = self.params.take();
        params.unwrap_or(vec![])
    }

    pub fn update_by(&mut self, path: PathStr, params: Option<Vec<FuncDataPathParam>>) {
        self.path = path;
        self.params = params;
    }
}

impl Display for FuncDataPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
