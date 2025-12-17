use crate::{FuncDataPathParam, ParserInnerState};

/// literal concat function parser.
///
/// it is always likely to be triggered only when encountering a [}],
/// and there may or may not be already parsed function parameters.
///
/// The parsing will continue until the following scenarios end:
/// - A matching [}] is always required to indicate the end of the current parameter.
///   Note that the current parameter refers to the part enclosed by [{}],
///   not all the parameters that the entire concat function can contain.
/// - After encountering the matching [}],
///   - if one of [.,)}] are encountered, the parsing of the entire function is considered complete.
///   - if [&] is encountered, a parsing error occurs,
///     since if a function appears in the literal concatenation function, it must be wrapped with [{}].
///   - if [(] is encountered, a parsing error occurs, since [(] must follow the function name.
///   - if [{] is encountered, next parameter starts,
///   - otherwise, next plain part starts.
pub struct LiteralConcatFuncParser<'a> {
    pub inner: ParserInnerState<'a>,
    pub params: Vec<FuncDataPathParam>,
}
