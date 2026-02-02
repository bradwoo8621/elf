use crate::{BaseDataModel, ModelErrorCode, Parameter, ParameterKind, Storable};
use elf_base::{ErrorCode, StdR};
use elf_model_marco::{adapt_model, Display, Serde, StrEnum, VPF};

// noinspection SpellCheckingInspection
/// predefined functions for variable parameters.
///
/// each function has restrictions:
/// - context: [context] refers to the execution context of a function.
///   For example, if it is [a.&b], then [a] is the execution context of function [b].
///   If a function allows a context, then when no execution context is provided,
///   the first parameter of the function is considered to be the execution context.
///   if false, the function is not allowed to have a `context`.
/// - min_param_count: minimum number of parameters the function requires.
/// - max_param_count: maximum number of parameters the function can accept, if 0, then no parameter allowed.
///   if None, then no limit.
///
/// difference with python implementation:
/// - most of the functions support none context now, to avoid the error in runtime.
#[derive(Display, Serde, StrEnum, VPF)]
#[pattern = "ampersand-prefix"]
pub enum VariablePredefineFunctions {
    // Sequence functions
    /// - get next sequence number,
    /// - [only in-memory].
    ///
    /// - [syntax]: \[&nextSeq], \[&nextSeq()]
    /// - [context]: not allowed,
    /// - [parameter]: not allowed.
    #[restrict(context = false, max_param_count = 0)]
    NextSeq,
    // Aggregation functions
    /// - count of vec or map,
    /// - when context is none, returns 0,
    /// - [only in-memory].
    ///
    /// - [syntax]: [x.&count], [x.&count()], \[&count(x)],
    /// - [context]: vec, map or none,
    /// - [parameter]: not allowed.
    #[restrict(none_context = true, max_param_count = 0)]
    Count,
    // String functions
    /// - chars count of string or decimal (to string),
    /// - when context is none, returns 0.
    ///
    /// - [syntax]: [x.&length], [x.&length()], \[&length(x)],
    /// - [context]: string, decimal or none,
    /// - [parameter]: not allowed.
    #[restrict(none_context = true, blank_context = true, max_param_count = 0)]
    Length,
    /// alias of [VariablePredefineFunctions::Length].
    ///
    /// - [syntax]: [x.&len], [x.&len()], \[&len(x)].
    #[restrict(none_context = true, blank_context = true, max_param_count = 0)]
    Len,
    /// - get substring from start (included) to end (excluded) of string,
    /// - when context is none, returns none.
    ///
    /// - [syntax]:
    ///   - from start (included) to end (excluded): [x.&slice(start, end)], [&slice(x, start, end)],
    ///   - from start (included) to end of string: [x.&slice(start)], [x.&slice(start, )], [&slice(x, start)], [&slice(x, start, )],
    ///   - from 0 to end (excluded): [x.&slice(, end)], [&slice(x, , end)],
    /// - [context]: string or none.
    /// - [parameter]:
    ///   - [start]: zero-based index, negative not allowed. none treat as 0.
    ///   - [end]: zero-based index, negative not allowed. none treat as maximum length of the string.
    ///     the maximum length of the string will be used as the limit when end is out of range.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 2
    )]
    Slice,
    /// alias of [VariablePredefineFunctions::Slice].
    ///
    /// - [syntax]:
    ///   - from start (included) to end (excluded): [x.&substr(start, end)], [&substr(x, start, end)],
    ///   - from start (included) to end of string: [x.&substr(start)], [x.&substr(start, )], [&substr(x, start)], [&substr(x, start, )]
    ///   - from 0 to end (excluded): [x.&substr(, end)], [&substr(x, , end)],
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 2
    )]
    Substr,
    /// - find start index of substring in string, -1 if not found.
    /// - when context is none, index of not empty substring is -1,
    /// - when context is none, index of empty substring is 0.
    ///
    /// - [syntax]: [x.&find(substring)], [&find(x, substring)]
    /// - [context]: string or none.
    /// - [parameter]:
    ///   - [substring]: string or none. if none, treat as empty string and returns 0.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 1
    )]
    Find,
    /// alias of [VariablePredefineFunctions::Find].
    ///
    /// - [syntax]: [x.&index(substring)], [&index(x, substring)]
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 1
    )]
    Index,
    /// - check if string starts with substring,
    /// - returns true when context is none and substring is empty string,
    /// - returns false when context is none and substring is not empty string.
    ///
    /// - [syntax]: [x.&startsWith(substring)], [&startsWith(x, substring)]
    /// - [context]: string or none.
    /// - [parameter]:
    ///   - [substring]: string or none. if none, treat as empty string and returns true.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 1
    )]
    StartsWith,
    /// alias of [VariablePredefineFunctions::StartsWith].
    ///
    /// - [syntax]: [x.&startswith(substring)], [&startswith(x, substring)]
    #[display = "&startswith"]
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 1
    )]
    Startswith,
    /// - check if string ends with substring,
    /// - returns true when context is none and substring is empty string,
    /// - returns false when context is none and substring is not empty string.
    ///
    /// - [syntax]: [x.&endsWith(substring)], [&endsWith(x, substring)]
    /// - [context]: string or none.
    /// - [parameter]:
    ///   - [substring]: string or none. if none, treat as empty string and returns true.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 1
    )]
    EndsWith,
    /// alias of [VariablePredefineFunctions::EndsWith].
    ///
    /// - [syntax]: [x.&endswith(substring)], [&endswith(x, substring)]
    #[display = "&endswith"]
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 1
    )]
    Endswith,
    /// - strip leading and trailing string (default whitespaces) from string,
    /// - returns empty string when context is none.
    ///
    /// - [syntax]:
    ///   - strip whitespaces: [x.&strip], [x.&strip()], [&strip(x)],
    ///   - strip given string: [x.&strip(stripString)], [&strip(x, stripString)]
    /// - [context]: string, or none.
    /// - [stripString]: string or none. if none, treat as whitespaces.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 0,
        max_param_count = 1
    )]
    Strip,
    /// alias of [VariablePredefineFunctions::Strip]
    ///
    /// - [syntax]:
    ///   - trim whitespaces: [x.&trim], [x.&trim()], [&trim(x)],
    ///   - trim given string: [x.&trim(trimString)], [&trim(x, trimString)]
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 0,
        max_param_count = 1
    )]
    Trim,
    /// - replace all occurrences of a substring with another substring in string,
    /// - when context is none, returns empty string.
    ///
    /// - [syntax]: [x.&replace(oldSubstring, newSubstring)], [&replace(x, oldSubstring, newSubstring)]
    /// - [context]: string or none.
    /// - [parameter]:
    ///   - [oldSubstring]: string or none. if none, treat as empty string.
    ///   - [newSubstring]: string or none. if none, treat as empty string.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 2,
        max_param_count = 2
    )]
    Replace,
    /// - replace first occurrence of a substring with another substring in string,
    /// - when context is none, returns empty string.
    ///
    /// - [syntax]: [x.&replaceFirst(oldSubstring, newSubstring)], [&replaceFirst(x, oldSubstring, newSubstring)]
    /// - [context]: string or none.
    /// - [parameter]
    ///   - [oldSubstring]: string or none. if none, treat as empty string.
    ///   - [newSubstring]: string or none. if none, treat as empty string.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 2,
        max_param_count = 2
    )]
    ReplaceFirst,
    /// convert string to upper case.
    /// - when context is none, return empty string.
    ///
    /// - [syntax]: [x.&upper], [x.&upper()], [&upper(x)]
    /// - [context]: string or none,
    /// - [parameter]: not allowed.
    #[restrict(none_context = true, blank_context = true, max_param_count = 0)]
    Upper,
    /// - convert string to lower case,
    /// - when context is none, return empty string.
    ///
    /// - [syntax]: [x.&lower], [x.&lower()], [&lower(x)]
    /// - [context]: string or none,
    /// - [parameter]: not allowed.
    #[restrict(none_context = true, blank_context = true, max_param_count = 0)]
    Lower,
    /// - check if string contains substring,
    /// - when context is none, returns true when param is empty string,
    /// - when context is none, returns false when param is not empty string.
    ///
    /// - [syntax]: [x.&contains(substring)], [&contains(x, substring)]
    /// - [context]: string or none.
    /// - [substring]: string or none. if none, treat as empty string.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 1
    )]
    Contains,
    /// - split string to vec by given separator string (default comma),
    /// - when context is none, returns a vec which has one empty string as the only element,
    ///
    /// - [syntax]:
    ///   - split by comma: [x.&split], [x.&split()], [&split(x)],
    ///   - split by separator: [x.&split(separator)], [&split(x, separator)].
    /// - [context]: string or none.
    /// - [parameter]:
    ///   - [separator]: string or none. if none, treat as comma.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 0,
        max_param_count = 1
    )]
    Split,
    /// - concatenate multiple strings to one string,
    /// - when context is none, treated as empty string.
    ///
    /// - [syntax]: [x.&concat(y, ...)], [&concat(x, y, ...)]
    /// - [context]: string or none.
    /// - [parameter]:
    ///   - [y, ...]: strings or nones. treated as empty string.
    #[restrict(none_context = true, blank_context = true, min_param_count = 1)]
    Concat,
    /// - concatenate multiple strings to one string with separator,
    /// - when context is none, treated as empty string.
    ///
    /// - [syntax]: [x.&concatWith(separator, y, ...)], [&concatWith(x, separator, y, ...)].
    /// - [context]: not vec/map values,
    /// - [parameter]:
    ///   - [separator]: not vec/map values. if none, treated as empty string.
    ///   - [y, ...]: not vec/map values. if none, treated as empty string
    ///
    /// return empty string when values are all none.
    #[restrict(none_context = true, blank_context = true, min_param_count = 2)]
    ConcatWith,
    // Vec functions
    /// - join the elements of vec to a string,
    /// - if context is not a vec, and not a map, convert to string,
    /// - if context is vec, each element in vec cannot be vec or map,
    /// - any none value of context, treated as empty string,
    /// - [only in-memory].
    ///
    /// - [syntax]:
    ///   - join with comma: [x.join], [x.&join()], [&join(x)]
    ///   - join with separator: [x.&join(separator)], [&join(x, separator)].
    /// - [context]: not map.
    /// - [parameter]:
    ///   - [separator]: string or none. if none, treat as comma.
    #[restrict(none_context = true, min_param_count = 0, max_param_count = 1)]
    Join,
    /// - get a distinct values of vec,
    /// - if context is not a vec, and not a map, returns a vec which has the context as only element,
    /// - if context is vec,
    ///   - if element is vec or map, always treated as distinct value,
    ///   - otherwise, compare with other, and leave the distinct value,
    /// - [only in-memory].
    ///
    /// - [syntax]: [x.&distinct], [x.&distinct()], [&distinct(x)]
    /// - [context]: not map,
    /// - [parameter]: not allowed.
    #[restrict(none_context = true, max_param_count = 0)]
    Distinct,
    /// - sum of elements of vec,
    /// - if context is none, returns 0,
    /// - if context is vec, each element should be a decimal, or a string can cast to decimal,
    /// - if element is none or empty string, treated as 0,
    /// - [only in-memory].
    ///
    /// - [syntax]: [x.&sum], [x.&sum()], [&sum(x)]
    /// - [context]: vec, none.
    #[restrict(none_context = true, max_param_count = 0)]
    Sum,
    /// - avg of elements of vec,
    /// - if context is none, returns none,
    /// - if context is vec, each element should be a decimal, or a string can cast to decimal,
    /// - if element is none or empty string, ignore,
    /// - if no valid element in vec, returns none,
    /// - [only in-memory].
    ///
    /// - [syntax]: [x.&avg], [x.&avg()], [&avg(x)]
    /// - [context]: vec, none.
    #[restrict(none_context = true, max_param_count = 0)]
    Avg,
    /// - max of elements of vec,
    /// - each value in vec should be
    ///   - a decimal/date/datetime/time,
    ///   - or can cast to decimal/date/datetime/time,
    ///   - or none which ignored,
    /// - all values must be same type,
    /// - date and datetime types are compatible,
    /// - if all elements are none or empty string, returns none,
    /// - [only in-memory].
    ///
    /// - [syntax]: [x.&max], [x.&max()], [&max(x)]
    /// - [context]: vec,
    /// - [parameter]: not allowed.
    ///
    /// e.g.
    /// - ["1", "1980-01-02", None] -> error, incompatible types decimal and date.
    /// - [1, 100, "204"] -> 204, string cast to decimal.
    /// - ["1980-01-02 12:23:45", "1979-11-30", None] -> "1980-01-02", datetime downgrade to date.
    /// - ["1979-11-30 12:23:45", "12:23:45"] -> error, incompatible types date/datetime and date.
    #[restrict(none_context = false, max_param_count = 0)]
    Max,
    /// - max decimal elements of vec,
    /// - each value in vec should be
    ///   - a decimal,
    ///   - or can cast to decimal,
    ///   - or none which ignored,
    /// - if all elements are none or empty string, returns none,
    /// - [only in-memory].
    ///
    /// - [syntax]: [x.&maxNum], [x.&maxNum()], [&maxNum(x)]
    /// - [context]: vec,
    /// - [parameter]: not allowed.
    #[restrict(none_context = false, max_param_count = 0)]
    MaxNum,
    /// - max date of elements of vec,
    /// - each value in vec should be
    ///   - a date/datetime,
    ///   - or can cast to date/datetime,
    ///   - or none which ignored,
    /// - if all elements are none or empty string, returns none,
    /// - [only in-memory].
    ///
    /// - [syntax]: [x.&maxDate], [x.&maxDate()], [&maxDate(x)]
    /// - [context]: vec,
    /// - [parameter]: not allowed.
    #[restrict(none_context = false, max_param_count = 0)]
    MaxDate,
    /// - max datetime of elements of vec,
    /// - each value in vec should be
    ///   - a date/datetime,
    ///   - or can cast to date/datetime,
    ///   - or none which ignored,
    /// - if all elements are none or empty string, returns none,
    /// - [only in-memory].
    ///
    /// - [syntax]: [x.&maxDatetime], [x.&maxDatetime()], [&maxDatetime(x)]
    /// - [context]: vec,
    /// - [parameter]: not allowed.
    #[restrict(none_context = false, max_param_count = 0)]
    MaxDatetime,
    /// alias of [VariablePredefineFunctions::MaxDatetime].
    ///
    /// - [syntax]: [x.&maxDt], [x.&maxDt()], [&maxDt(x)]
    #[restrict(none_context = false, max_param_count = 0)]
    MaxDt,
    /// - max time of elements of vec,
    /// - each value in vec should be
    ///   - a time,
    ///   - or can cast to time,
    ///   - or none which ignored,
    /// - if all elements are none or empty string, returns none,
    /// - [only in-memory].
    ///
    /// - [syntax]: [x.&maxTime], [x.&maxTime()], [&maxTime(x)]
    /// - [context]: vec,
    /// - [parameter]: not allowed.
    #[restrict(none_context = false, max_param_count = 0)]
    MaxTime,
    /// - min of elements of vec,
    /// - each value in vec should be
    ///   - a decimal/date/datetime/time,
    ///   - or can cast to decimal/date/datetime/time,
    ///   - or none which ignored,
    /// - all values must be same type,
    /// - date and datetime types are compatible,
    /// - if any element is none or empty string, returns none,
    /// - [only in-memory].
    ///
    /// - [syntax]: [x.&min], [x.&min()], [&min(x)]
    /// - [context]: vec,
    /// - [parameter]: not allowed.
    ///
    /// e.g.
    /// - ["1", "1980-01-02", None] -> error, incompatible types decimal and date.
    /// - [1, 100, "204"] -> 1, string cast to decimal.
    /// - ["1980-01-02", "1979-11-30 12:23:45", None] -> "1979-11-30", datetime downgrade to date.
    /// - ["1979-11-30 12:23:45", "12:23:45"] -> error, incompatible types date/datetime and date.
    #[restrict(none_context = false, max_param_count = 0)]
    Min,
    /// - min decimal elements of vec,
    /// - each value in vec should be
    ///   - a decimal,
    ///   - or can cast to decimal,
    ///   - or none which ignored,
    /// - if any element is none or empty string, returns none,
    /// - [only in-memory].
    ///
    /// - [syntax]: [x.&minNum], [x.&minNum()], [&minNum(x)]
    /// - [context]: vec,
    /// - [parameter]: not allowed.
    #[restrict(none_context = false, max_param_count = 0)]
    MinNum,
    /// - min date of elements of vec,
    /// - each value in vec should be
    ///   - a date/datetime,
    ///   - or can cast to date/datetime,
    ///   - or none which ignored,
    /// - if any element is none or empty string, returns none,
    /// - [only in-memory].
    ///
    /// - [syntax]: [x.&minDate], [x.&minDate()], [&minDate(x)]
    /// - [context]: vec,
    /// - [parameter]: not allowed.
    #[restrict(none_context = false, max_param_count = 0)]
    MinDate,
    /// - min date time of elements of vec,
    /// - each value in vec should be
    ///   - a date/datetime,
    ///   - or can cast to date/datetime,
    ///   - or none which ignored,
    /// - if any element is none or empty string, returns none,
    /// - [only in-memory].
    ///
    /// - [syntax]: [x.&minDatetime], [x.&minDatetime()], [&minDatetime(x)]
    /// - [context]: vec,
    /// - [parameter]: not allowed.
    #[restrict(none_context = false, max_param_count = 0)]
    MinDatetime,
    /// alias of [VariablePredefineFunctions::MinDatetime].
    ///
    /// - [syntax]: [x.&minDt], [x.&minDt()], [&minDt(x)]
    #[restrict(none_context = false, max_param_count = 0)]
    MinDt,
    /// - min time of elements of vec,
    /// - each value in vec should be
    ///   - a time,
    ///   - or can cast to time,
    ///   - or none which ignored,
    /// - if any element is none or empty string, returns none,
    /// - [only in-memory].
    ///
    /// - [syntax]: [x.&minTime], [x.&minTime()], [&minTime(x)]
    /// - [context]: vec,
    /// - [parameter]: not allowed.
    #[restrict(none_context = false, max_param_count = 0)]
    MinTime,
    /// - retrieve value from current context, include variables and current trigger data,
    /// - [only in-memory].
    ///
    /// - [syntax]: [&cur], [&cur()],
    /// - [context]: not allowed,
    /// - [parameter]: not allowed.
    #[display = "&cur"]
    #[restrict(context = false, max_param_count = 0)]
    FromCurrentTriggerData,
    /// - retrieve value from previous trigger data,
    /// - [only in-memory].
    ///
    /// - [syntax]: [&old], [&old()],
    /// - [context]: not allowed,
    /// - [parameter]: not allowed.
    #[display = "&old"]
    #[restrict(context = false, max_param_count = 0)]
    FromPreviousTriggerData,
    // Date related functions
    /// - get day difference between context and parameter.
    /// - time part of datetime ignored.
    ///
    /// - [syntax]: [x.&dayDiff(otherDate)], [&dayDiff(x, otherDate)]
    /// - [context]: date/datetime, string can cast to date/datetime.
    /// - [parameter]:
    ///   - [otherDate]: date/datetime, string can cast to date/datetime.
    #[restrict(none_context = false, min_param_count = 1, max_param_count = 1)]
    DayDiff,
    /// - get month difference between context and parameter.
    /// - time part of datetime ignored.
    ///
    /// - [syntax]: [x.&monthDiff(otherDate)], [&monthDiff(x, otherDate)]
    /// - [context]: date/datetime, string can cast to date/datetime.
    /// - [parameter]:
    ///   - [otherDate]: date/datetime, string can cast to date/datetime.
    #[restrict(none_context = false, min_param_count = 1, max_param_count = 1)]
    MonthDiff,
    /// - get year difference between context and parameter.
    /// - time part of datetime ignored.
    ///
    /// - [syntax]: [x.&yearDiff(otherDate)], [&yearDiff(x, otherDate)]
    /// - [context]: date/datetime, string can cast to date/datetime.
    /// - [parameter]:
    ///   - [otherDate]: date/datetime, string can cast to date/datetime.
    #[restrict(none_context = false, min_param_count = 1, max_param_count = 1)]
    YearDiff,
    /// - move date by given days, months, years.
    /// - if context has no corresponding part of movement, ignore the movement.
    ///
    /// - [syntax]: [x.&moveDate(movement)], [&moveDate(x, movement)]
    /// - [context]: date/datetime, string can cast to date/datetime.
    /// - [parameter]:
    ///   - [movement]: string. format:
    ///     - unit: YMDhms,
    ///     - positive/negative: +/-, optional,
    ///     - if 2nd part is +/-, any number value; or
    ///       - year(Y): 4 digits year,
    ///	  	  - month(M): 1 - 12. any value not in [1, 12] will be normalized to [1, 12],
    ///	  	  - date(D): 1 - end of month (28/29/30/31). 99 means end of month,
    ///           otherwise any value not in [1, end of month] will be normalized to [1, end of month],
    ///	  	  - hour(h): 0 - 23. any value not in [0, 23] will be normalized to [0, 23],
    ///	  	  - minute(m): 0 - 59. any value not in [0, 59] will be normalized to [0, 59],
    ///	  	  - second(s): 0 - 59. any value not in [0, 59] will be normalized to [0, 59],
    ///     - whitespaces between 3 parts are allowed, and ignored.
    ///
    /// e.g. [date.&moveDate(Y2000M+1D-1h23m+5s-6)],
    /// if date is 1999-11-30, then result is 2000-12-29 23:04:54:
    /// - year set to 2000, now is 2000-11-30,
    /// - month plus 1, to 12, now is 2000-12-30,
    /// - day minus 1, to 29, now is 2000-12-29,
    /// - hour set to 23, now is 2000-12-29 23:00:00 (original no time, default is 00:00:00),
    /// - minute plus 5, to 5, now is 2000-12-29 23:05:00,
    /// - second minus 6, to 54, and minute minus 1. result is 2000-12-29 23:04:54.
    #[restrict(none_context = false, min_param_count = 1, max_param_count = 1)]
    MoveDate,
    /// - format date to string by given format.
    ///
    /// - [syntax]: [x.&dateFormat(format)], [&dateFormat(x, format)]
    /// - [context]: date/datetime, string can cast to date/datetime.
    /// - [parameter]:
    ///   - [format]: string. date format.
    ///     - 'Y': '%Y',  # 4 digits year
    ///     - 'y': '%y',  # 2 digits year
    ///     - 'M': '%m',  # 2 digits month
    ///     - 'D': '%d',  # 2 digits day of month
    ///     - 'h': '%H',  # 2 digits hour, 00 - 23
    ///     - 'H': '%I',  # 2 digits hour, 01 - 12
    ///     - 'm': '%M',  # 2 digits minute
    ///     - 's': '%S',  # 2 digits second
    ///     - 'W': '%A',  # Monday - Sunday
    ///     - 'w': '%a',  # Mon - Sun
    ///     - 'B': '%B',  # January - December
    ///     - 'b': '%b',  # Jan - Dec
    ///	    - 'p': '%p'  # AM/PM
    ///
    /// e.g. [date.&fmtDate(%Y-%M-%D)],
    /// if date is 2000-12-29, then result is 2000-12-29 00:00:00.
    #[display = "&fmtDate"]
    #[restrict(none_context = false, min_param_count = 1, max_param_count = 1)]
    DateFormat,
    /// - get current date time.
    ///
    /// - [syntax]: [&now], [&now()]
    /// - [context]: not allowed,
    /// - [parameter]: not allowed.
    #[restrict(context = false, max_param_count = 0)]
    Now,
}

impl VariablePredefineFunctions {
    // noinspection DuplicatedCode
    /// whether the function allow none as parameter.
    /// returns false if the function given parameter index is not accepted (over max param count)
    /// or the function does not accept none at given parameter index.
    /// context parameter is not included.
    pub fn allow_none_param(&self, param_index: usize) -> bool {
        if let Some(max_param_count) = self.max_param_count() {
            if param_index > max_param_count {
                return false;
            }
        }

        // function does not accept any parameter, false
        let no_param_false = false;
        // function does accept single parameter, but it cannot be blank/empty string, false
        let single_param_false = false;
        // function does accept single parameter, and it can be blank/empty string, true
        let single_param_true = true;
        // function does accept two parameters, and both can be blank/empty string, true
        let both_params_true = true;
        // function does accept multiple parameters, and any can be blank/empty string, true
        let any_param_true = true;

        match self {
            Self::NextSeq => no_param_false,
            Self::Count => no_param_false,
            Self::Length => no_param_false,
            Self::Len => no_param_false,
            Self::Slice => both_params_true,
            Self::Substr => both_params_true,
            Self::Find => single_param_true,
            Self::Index => single_param_true,
            Self::StartsWith => single_param_true,
            Self::Startswith => single_param_true,
            Self::EndsWith => single_param_true,
            Self::Endswith => single_param_true,
            Self::Strip => single_param_true,
            Self::Trim => single_param_true,
            Self::Replace => both_params_true,
            Self::ReplaceFirst => both_params_true,
            Self::Upper => no_param_false,
            Self::Lower => no_param_false,
            Self::Contains => single_param_true,
            Self::Split => single_param_true,
            Self::Concat => any_param_true,
            Self::ConcatWith => any_param_true,
            Self::Join => single_param_true,
            Self::Distinct => no_param_false,
            Self::Sum => no_param_false,
            Self::Avg => no_param_false,
            Self::Max => no_param_false,
            Self::MaxNum => no_param_false,
            Self::MaxDate => no_param_false,
            Self::MaxDatetime => no_param_false,
            Self::MaxDt => no_param_false,
            Self::MaxTime => no_param_false,
            Self::Min => no_param_false,
            Self::MinNum => no_param_false,
            Self::MinDate => no_param_false,
            Self::MinDatetime => no_param_false,
            Self::MinDt => no_param_false,
            Self::MinTime => no_param_false,
            Self::FromCurrentTriggerData => no_param_false,
            Self::FromPreviousTriggerData => no_param_false,
            Self::DayDiff => single_param_false,
            Self::MonthDiff => single_param_false,
            Self::YearDiff => single_param_false,
            Self::MoveDate => single_param_false,
            Self::DateFormat => single_param_false,
            Self::Now => no_param_false,
        }
    }

    // noinspection DuplicatedCode
    /// whether the function allow blank or empty string as parameter.
    /// returns false if the function given parameter index is not accepted (over max param count)
    /// or the function does not accept string parameter at given parameter index.
    /// context parameter is not included.
    pub fn allow_blank_param(&self, param_index: usize) -> bool {
        if let Some(max_param_count) = self.max_param_count() {
            if param_index > max_param_count {
                return false;
            }
        }

        // function does not accept any parameter, false
        let no_param_false = false;
        // function does accept single parameter, but it cannot be blank/empty string, false
        let single_param_false = false;
        // function does accept single parameter, and it can be blank/empty string, true
        let single_param_true = true;
        // function does accept two parameters, but both cannot be blank/empty string, false
        let both_params_false = false;
        // function does accept two parameters, and both can be blank/empty string, true
        let both_params_true = true;
        // function does accept multiple parameters, and any can be blank/empty string, true
        let any_param_true = true;

        match self {
            Self::NextSeq => no_param_false,
            Self::Count => no_param_false,
            Self::Length => no_param_false,
            Self::Len => no_param_false,
            Self::Slice => both_params_false,
            Self::Substr => both_params_false,
            Self::Find => single_param_true,
            Self::Index => single_param_true,
            Self::StartsWith => single_param_true,
            Self::Startswith => single_param_true,
            Self::EndsWith => single_param_true,
            Self::Endswith => single_param_true,
            Self::Strip => single_param_true,
            Self::Trim => single_param_true,
            Self::Replace => both_params_true,
            Self::ReplaceFirst => both_params_true,
            Self::Upper => no_param_false,
            Self::Lower => no_param_false,
            Self::Contains => single_param_true,
            Self::Split => single_param_true,
            Self::Concat => any_param_true,
            Self::ConcatWith => any_param_true,
            Self::Join => single_param_true,
            Self::Distinct => no_param_false,
            Self::Sum => no_param_false,
            Self::Avg => no_param_false,
            Self::Max => no_param_false,
            Self::MaxNum => no_param_false,
            Self::MaxDate => no_param_false,
            Self::MaxDatetime => no_param_false,
            Self::MaxDt => no_param_false,
            Self::MaxTime => no_param_false,
            Self::Min => no_param_false,
            Self::MinNum => no_param_false,
            Self::MinDate => no_param_false,
            Self::MinDatetime => no_param_false,
            Self::MinDt => no_param_false,
            Self::MinTime => no_param_false,
            Self::FromCurrentTriggerData => no_param_false,
            Self::FromPreviousTriggerData => no_param_false,
            Self::DayDiff => single_param_false,
            Self::MonthDiff => single_param_false,
            Self::YearDiff => single_param_false,
            Self::MoveDate => single_param_false,
            Self::DateFormat => single_param_false,
            Self::Now => no_param_false,
        }
    }
}

/// string stands for an expression to retrieve some value
/// might include function calls, see [VariablePredefineFunctions]
#[adapt_model(storable)]
pub struct ConstantParameter {
    pub kind: Option<ParameterKind>,
    pub value: Option<String>,
}

impl ConstantParameter {
    pub fn init() -> Self {
        Self::new().kind(ParameterKind::Constant)
    }

    pub fn of(value: String) -> Self {
        Self::init().value(value)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Constant(self)
    }
}
