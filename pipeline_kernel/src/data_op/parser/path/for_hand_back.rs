use crate::{
    DataPath, DataPathSegment, FuncDataPathParam, FuncParamValue, FuncParamValuePath, PathParser,
};

pub trait AnyFuncParser {
    fn append_param(&mut self, param: FuncDataPathParam);

    fn param_start_char_index(&self) -> usize;

    fn move_char_index_to(&mut self, char_index: usize);
}

impl PathParser {
    /// there are 2 options of handing back to func parsing,
    /// - back to standard function parsing,
    /// - back to literal concat function parsing,
    /// the difference of these 2 options are, in path parser,
    /// - for parsing path for standard function parameter, end at one of [,)], and char index stay at [,)],
    /// - for parsing path for literal concat function, end at [}], and char index stay at next of ending [}],
    /// basically, this difference will affect the starting character position of the subsequent parsing.
    /// however, this is an issue that the subsequent parser needs to pay attention to.
    /// here, this difference will affect the ending position of the parsed parameters.
    /// since end char [,)}] need to be excluded from the parameter,
    /// there will be some differences in calculating the ending position of the parameters between the two methods.
    /// specifically, there will be an offset of [0] (standard) or [-1] (literal concat) for the end char index.
    fn hand_back_to(self, end_char_index_offset: usize, func: &mut impl AnyFuncParser) {
        let mut segments = self.segments;
        if segments.is_empty() {
            // for literal concat function,
            // no segment, basically, it is a "{}", treated as an empty string
            func.append_param(FuncDataPathParam::Value(FuncParamValuePath {
                path: self.inner.create_path_str(
                    func.param_start_char_index(),
                    self.inner.current_char_index() - end_char_index_offset,
                ),
                value: FuncParamValue::Str(String::from("")),
            }))
        } else if segments.len() > 1 {
            func.append_param(FuncDataPathParam::Path(DataPath {
                path: self.inner.create_path_str(
                    func.param_start_char_index(),
                    self.inner.current_char_index() - end_char_index_offset,
                ),
                segments,
            }))
        } else {
            match segments.pop().unwrap() {
                DataPathSegment::Plain(plain_path) => {
                    func.append_param(FuncDataPathParam::Plain(plain_path))
                }
                DataPathSegment::Func(func_path) => {
                    func.append_param(FuncDataPathParam::Func(func_path))
                }
            }
        }
        // copy char index to current state
        func.move_char_index_to(self.inner.current_char_index());
    }

    /// hand back to standard function parser
    /// when back, the char index is at one of [,)]
    pub fn hand_back_to_func(self, func: &mut impl AnyFuncParser) {
        self.hand_back_to(0, func)
    }

    /// hand back to literal concat parser.
    /// when back, the char index is after the closing "}".
    pub fn hand_back_to_literal_concat(self, func: &mut impl AnyFuncParser) {
        self.hand_back_to(1, func)
    }
}
