use crate::{
    DataPathSegment, FuncDataPath, FuncDataPathParam, FuncParamValue, FuncParamValuePath,
    LiteralConcatFuncParser, ParserInnerState, PathParser, PathStr,
};
use elf_base::{StdR, VoidR};
use elf_model::VariablePredefineFunctions;

/// for literal concat function
impl PathParser {
    /// get index of char before on-flying chars, when a literal concat function detected.
    /// current char is [{].
    ///
    /// if there is chars in in-memory chars, before it,
    /// otherwise previous char index
    fn get_index_of_char_before_literal_concat_function(&self) -> i64 {
        if self.inner.in_memory_chars_is_not_empty() {
            // get char before in-memory chars
            self.inner
                .char_index_before_current(self.inner.in_memory_chars_count() + 1)
        } else {
            // get char before "{"
            self.inner.previous_char_index()
        }
    }

    /// if char before in-memory chars (if exists) or [{] is
    /// - one of [.,(] or start of full path: should create,
    /// - [}], not create,
    /// - otherwise raise error
    fn should_create_concat_function(&self, index_of_char_before: i64) -> StdR<bool> {
        let inner = &self.inner;

        let char_before: Option<&char> = inner.char_at(index_of_char_before);
        if let Some(char_before) = char_before {
            match char_before {
                '.' | ',' | '(' => Ok(true),
                '}' => Ok(false),
                _ => self.inner.incorrect_left_brace()?,
            }
        } else {
            // no char before, it is the first part
            Ok(true)
        }
    }

    /// - if [should_create_concat_function] is true, create a concat function and return.
    ///   [path] and [params] of returned function is empty, since consuming not start yet.
    /// - or pop last element from segments, and return.
    ///   note if the last element is not a concat function, or segments is empty, raise error.
    fn get_or_create_concat_function(
        &mut self,
        should_create_concat_function: bool,
    ) -> StdR<FuncDataPath> {
        if should_create_concat_function {
            Ok(FuncDataPath {
                // leave the path empty, will set it later
                path: PathStr::of_str(""),
                func: VariablePredefineFunctions::Concat,
                params: Some(vec![]),
            })
        } else if let Some(data_path) = self.pop_last_concat_function() {
            // no need to create, and there is no segment in state, something wrong here!
            Ok(data_path)
        } else {
            self.inner.incorrect_left_brace()
        }
    }

    /// left brace starts a sub path, it must be part of literal concat function.
    /// considering the following scenarios, for all following cases, [{ccc}] is sub path waiting for consuming:
    /// - not first part of concat function,
    ///   - [aaa{ccc}]: [aaa] already consumed, last segment in state is a plain path,
    ///   - [aaa{bbb}{ccc}]: [aaa{bbb}] already consumed, last segment in state is concat function path,
    /// - is first part of concat function,
    ///   - [aaa.{ccc}]: [aaa.] already consumed, last segment in state is a plain path,
    ///   - [aaa{bbb}.{ccc}]: [aaa{bbb}.] already consumed, last segment in state is concat function path,
    ///   - [&now.{ccc}]: [&now.] already consumed, last segment in state is a function path, but not concat,
    ///   - [&len({ccc})]: [&len(] already consumed, segment is empty,
    ///   - [&yearDiff(abc,{ccc}]: [[&yearDiff(abc, ] already consumed, segment is empty,
    /// and current char index is at the [{] of [{ccc}].
    ///
    /// so, need to check followings,
    /// - there are chars in in-memory chars: not first part, then check the char before in-memory chars,
    ///   - is one of [.,(] or start of full path:
    ///     - create concat function,
    ///     - let in-memory chars to be a string parameter part, is the first param of concat,
    ///   - is [}]: then check the last segment,
    ///     - is a concat function, use it,
    ///     - is not a concat function, raise error (logically never happens)
    ///   - is not one of [.,(}], raise error
    /// - the char before [{] is one of [.,(] or start of full path: is first part,
    ///   - create concat function,
    /// - the char before [{] is [}]: not first part, then check the last segment,
    ///   - is a concat function, use it,
    ///   - is not a concat function, raise error (logically never happens)
    /// - the char before [{] not one of [.,(}], raise error
    pub fn consume_literal_concat_function(&mut self) -> VoidR {
        let index_of_char_before = self.get_index_of_char_before_literal_concat_function();
        let should_create_concat_function =
            self.should_create_concat_function(index_of_char_before)?;
        let mut concat = self.get_or_create_concat_function(should_create_concat_function)?;
        // move all segments from existing concat function,
        // now the concat is empty
        let params = concat.params.take();
        let mut params = params.unwrap_or(vec![]);

        if self.inner.in_memory_chars_is_not_empty() {
            // create a value path
            let value = FuncParamValue::Str(self.inner.clone_in_memory_chars());
            let start_char_index = self
                .inner
                .char_index_before_current(self.inner.in_memory_chars_count())
                as usize;
            params.push(FuncDataPathParam::Value(FuncParamValuePath {
                path: self.inner.create_path_str_exclude_current(start_char_index),
                value,
            }));
            self.inner.clear_in_memory_chars();
        }

        // continue parsing literal concat function
        let mut literal_concat_func_parser = LiteralConcatFuncParser {
            inner: ParserInnerState::new_at_next_char(&self.inner),
            params,
        };
        literal_concat_func_parser.parse()?;
        // hand back
        // copy char index to current state
        self.inner
            .move_char_index_to(literal_concat_func_parser.inner.current_char_index());
        // reset the concat function path
        concat.path = self
            .inner
            .create_path_str_exclude_current((index_of_char_before + 1) as usize);
        // copy params to concat function
        concat.params = Some(literal_concat_func_parser.params);

        // append the concat function to segments
        self.append_segment(DataPathSegment::Func(concat));

        Ok(())
    }
}
