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
#[derive(Display, Serde, StrEnum, VPF)]
#[pattern = "ampersand-prefix"]
pub enum VariablePredefineFunctions {
    // Sequence functions
    /// get next sequence number, [only in-memory]. \[&nextSeq], \[&nextSeq()]
    #[restrict(context = false, max_param_count = 0)]
    NextSeq,
    // Aggregation functions
    /// count of vec or map, [only in-memory]. [x.&count], [x.&count()], \[&count(x)]
    ///
    /// - [context]: vec, map or none.
    /// - [none context]: returns 0.
    #[restrict(none_context = true, max_param_count = 0)]
    Count,
    // String functions
    /// chars count of string or decimal (to string). [x.&length], [x.&length()], \[&length(x)]
    ///
    /// - [context]: string, decimal or none.
    /// - [none context]: returns 0.
    #[restrict(none_context = true, blank_context = true, max_param_count = 0)]
    Length,
    /// alias of [VariablePredefineFunctions::Length]. [x.&len], [x.&len()], [&len(x)]
    #[restrict(none_context = true, blank_context = true, max_param_count = 0)]
    Len,
    /// get substring of string.
    /// - from start (included) to end (excluded): [x.&slice(start, end)], [&slice(x, start, end)],
    /// - from start (included) to end of string: [x.&slice(start)], [x.&slice(start, )], [&slice(x, start)], [&slice(x, start, )]
    /// - from 0 to end (excluded): [x.&slice(, end)], [&slice(x, , end)],
    ///
    /// - [context]: string, none.
    /// - [none context]: returns empty string.
    /// - [start]: zero-based index, negative not allowed. none treat as 0.
    /// - [end]: zero-based index, negative not allowed. none treat as maximum length of the string.
    ///   the maximum length of the string will be used as the limit when end is out of range.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 2
    )]
    Slice,
    /// alias of [VariablePredefineFunctions::Slice].
    /// - from start (included) to end (excluded): [x.&substr(start, end)], [&substr(x, start, end)],
    /// - from start (included) to end of string: [x.&substr(start)], [x.&substr(start, )], [&substr(x, start)], [&substr(x, start, )]
    /// - from 0 to end (excluded): [x.&substr(, end)], [&substr(x, , end)],
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 2
    )]
    Substr,
    /// find substring in string, return the start index, -1 if not found.
    /// [x.&find(substring)], [&find(x, substring)]
    ///
    /// - [context]: string, none.
    /// - [none context]:
    ///   - returns -1 when substring is not empty.
    ///   - returns 0 when substring is empty or none.
    /// - [substring]: string, none. if none, treat as empty string and returns 0.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 1
    )]
    Find,
    /// alias of [VariablePredefineFunctions::Find].
    /// [x.&index(substring)], [&index(x, substring)]
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 1
    )]
    Index,
    /// check if string starts with substring, return boolean.
    /// [x.&startsWith(substring)], [&startsWith(x, substring)]
    ///
    /// - [context]: string, none.
    /// - [none context] returns true.
    /// - [substring]: string, none. if none, treat as empty string and returns true.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 1
    )]
    StartsWith,
    /// alias of [VariablePredefineFunctions::StartsWith].
    /// [x.&startswith(substring)], [&startswith(x, substring)]
    #[display = "&startswith"]
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 1
    )]
    Startswith,
    /// check if string ends with substring, return boolean.
    /// [x.&endsWith(substring)], [&endsWith(x, substring)]
    ///
    /// - [context]: string, none.
    /// - [none context] returns true.
    /// - [substring]: string, none. if none, treat as empty string and returns true.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 1
    )]
    EndsWith,
    /// alias of [VariablePredefineFunctions::EndsWith].
    /// [x.&endswith(substring)], [&endswith(x, substring)]
    #[display = "&endswith"]
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 1
    )]
    Endswith,
    /// strip leading and trailing string (default whitespaces) from string.
    /// - strip whitespaces: [x.&strip], [x.&strip()], [&strip(x)],
    /// - strip given string: [x.&strip(stripString)], [&strip(x, stripString)]
    ///
    /// - [context]: string, none.
    /// - [none context] returns empty string.
    /// - [stripString]: string, none. if none, treat as whitespaces.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 0,
        max_param_count = 1
    )]
    Strip,
    /// alias of [VariablePredefineFunctions::Strip]
    /// - trim whitespaces: [x.&trim], [x.&trim()], [&trim(x)],
    /// - trim given string: [x.&trim(trimString)], [&trim(x, trimString)]
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 0,
        max_param_count = 1
    )]
    Trim,
    /// replace all occurrences of a substring with another substring in string.
    /// [x.&replace(oldSubstring, newSubstring)], [&replace(x, oldSubstring, newSubstring)]
    ///
    /// - [context]: string, none.
    /// - [none context] returns empty string.
    /// - [oldSubstring]: string, none. if none, treat as empty string.
    /// - [newSubstring]: string, none. if none, treat as empty string.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 2,
        max_param_count = 2
    )]
    Replace,
    /// replace first occurrence of a substring with another substring in string.
    /// [x.&replaceFirst(oldSubstring, newSubstring)], [&replaceFirst(x, oldSubstring, newSubstring)]
    ///
    /// - [context]: string, none.
    /// - [none context] returns empty string.
    /// - [oldSubstring]: string, none. if none, treat as empty string.
    /// - [newSubstring]: string, none. if none, treat as empty string.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 2,
        max_param_count = 2
    )]
    ReplaceFirst,
    /// convert string to upper case.
    /// [x.&upper], [x.&upper()], [&upper(x)]
    ///
    /// - [context]: string, none.
    /// - [none context] returns empty string.
    #[restrict(none_context = true, blank_context = true, max_param_count = 0)]
    Upper,
    /// convert string to lower case.
    /// [x.&lower], [x.&lower()], [&lower(x)]
    ///
    /// - [context]: string, none.
    /// - [none context] returns empty string.
    #[restrict(none_context = true, blank_context = true, max_param_count = 0)]
    Lower,
    /// check if string contains substring, return boolean.
    /// [x.&contains(substring)], [&contains(x, substring)]
    ///
    /// - [context]: string, none.
    /// - [none context]:
    ///   - returns false when given substring is not empty.
    ///   - returns true when given substring is empty,
    /// - [substring]: string, none. if none, treat as empty string.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 1,
        max_param_count = 1
    )]
    Contains,
    /// split string to vec by given separator string (default comma).
    /// - split by comma: [x.&split], [x.&split()], [&split(x)],
    /// - split by separator: [x.&split(separator)], [&split(x, separator)].
    ///
    /// - [context]: string, none.
    /// - [none context]
    ///   - returns vec with single empty string element when given separator is not empty.
    ///   - returns empty vec when given separator is empty,
    /// - [separator]: string, none. if none, treat as comma.
    #[restrict(
        none_context = true,
        blank_context = true,
        min_param_count = 0,
        max_param_count = 1
    )]
    Split,
    /// concatenate multiple strings to one string.
    /// - [x.&concat(y, ...)], [&concat(x, y, ...)]
    ///
    /// - [context]: string, none.
    /// - [none context], simply ignore it.
    /// - [y, ...]: strings or nones. none values are ignored.
    ///
    /// return empty string when values are all none.
    #[restrict(none_context = true, blank_context = true, min_param_count = 1)]
    Concat,
    /// concatenate multiple strings in vec to one string with separator
    /// - [x.&concatWith(separator, y, ...)], [&concatWith(x, separator, y, ...)].
    ///
    /// - [context]: not vec/map values. none treated as empty string
    /// - [separator]: not vec/map values. none value is treated as empty string.
    /// - [y, ...]: not vec/map values. none treated as empty string
    ///
    /// return empty string when values are all none.
    #[restrict(none_context = true, blank_context = true, min_param_count = 2)]
    ConcatWith,
    /// join the elements of vec to a string, [only in-memory]
    /// - join with comma: [x.join], [x.&join()], [&join(x)]
    /// - join with separator: [x.&join(separator)], [&join(x, separator)].
    ///
    /// - [context]: non map.
    /// - [none context], returns empty string.
    /// - [separator]: string, none. if none, treat as comma.
    ///
    /// none values in vec are ignored. return empty string when values are all none.
    #[restrict(none_context = true, min_param_count = 0, max_param_count = 1)]
    Join,
    // Statistical functions
    /// get a distinct vec, [only in-memory].
    /// [x.&distinct], [x.&distinct()], [&distinct(x)]
    ///
    /// - [context]: non map.
    /// - [none context], returns empty vec.
    #[restrict(none_context = true, max_param_count = 0)]
    Distinct,
    /// sum of elements of vec, [only in-memory].
    /// [x.&sum], [x.&sum()], [&sum(x)]
    ///
    /// - [context]: vec, none.
    /// - [none context], returns 0.
    ///
    /// each value in vec should be a decimal, or can cast to decimal,
    /// or none/empty string which treated as 0,
    /// otherwise error raised.
    #[restrict(none_context = true, max_param_count = 0)]
    Sum,
    /// avg of elements of vec, [only in-memory].
    /// [x.&avg], [x.&avg()], [&avg(x)]
    ///
    /// - [context]: vec, none.
    /// - [none context], returns 0.
    ///
    /// each value in vec should be a decimal, or can cast to decimal,
    /// otherwise error raised.
    /// none/empty string doesn't count in
    #[restrict(none_context = true, max_param_count = 0)]
    Avg,
    /// max of elements of vec, [only in-memory].
    /// [x.&max], [x.&max()], [&max(x)]
    ///
    /// - [context]: vec, none.
    /// - [none context], returns 0.
    ///
    /// each value in vec should be a decimal/date/datetime/time,
    /// or can cast to decimal/date/datetime/tim, or none which ignored,
    /// otherwise error raised.
    /// and all values must be same type, except datetime will be automatically cast to date if there is date type existing,
    /// if all values are none, returns none.
    ///
    /// e.g.
    /// - ["1", "1980-01-02", None] -> error, incompatible types decimal and date.
    /// - [1, 100, "204"] -> 204, string cast to decimal.
    /// - ["1980-01-02 12:23:45", "1979-11-30", None] -> "1980-01-02", datetime downgrade to date.
    /// - ["1979-11-30 12:23:45", "12:23:45"] -> error, incompatible types date/datetime and date.
    #[restrict(none_context = true, max_param_count = 0)]
    Max,
    /// max decimal elements of vec, [only in-memory].
    /// [x.&maxNum], [x.&maxNum()], [&maxNum(x)]
    ///
    /// - [context]: vec, none.
    /// - [none context], returns none.
    ///
    /// each value in vec should be a decimal, or can cast to decimal, or none which ignored,
    /// otherwise error raised.
    /// if all values are none, returns none.
    #[restrict(none_context = true, max_param_count = 0)]
    MaxNum,
    /// max date of elements of vec, [only in-memory].
    /// [x.&maxDate], [x.&maxDate()], [&maxDate(x)]
    ///
    /// - [context]: vec, none.
    /// - [none context], returns none.
    ///
    /// each value in vec should be a date/datetime, or can cast to date/datetime, or none which ignored,
    /// otherwise error raised.
    /// if all values are none, returns none.
    #[restrict(none_context = true, max_param_count = 0)]
    MaxDate,
    /// max date time of elements of vec, [only in-memory].
    /// [x.&maxDatetime], [x.&maxDatetime()], [&maxDatetime(x)]
    ///
    /// - [context]: vec, none.
    /// - [none context], returns none.
    ///
    /// each value in vec should be a datetime, or can cast to datetime, or none which ignored,
    /// otherwise error raised.
    /// if all values are none, returns none.
    #[restrict(none_context = true, max_param_count = 0)]
    MaxDatetime,
    /// alias of [VariablePredefineFunctions::MaxDatetime].
    /// [x.&maxDt], [x.&maxDt()], [&maxDt(x)]
    #[restrict(none_context = true, max_param_count = 0)]
    MaxDt,
    /// max time of elements of vec, [only in-memory].
    /// [x.&maxTime], [x.&maxTime()], [&maxTime(x)]
    ///
    /// - [context]: vec, none.
    /// - [none context], returns none.
    ///
    /// each value in vec should be a time, or can cast to time, or none which ignored,
    /// otherwise error raised.
    /// if all values are none, returns none.
    #[restrict(none_context = true, max_param_count = 0)]
    MaxTime,
    /// min of elements of vec, [only in-memory].
    /// [x.&min], [x.&min()], [&min(x)]
    ///
    /// - [context]: vec, none.
    /// - [none context], returns none.
    ///
    /// each value in vec should be a decimal/date/datetime/time,
    /// or can cast to decimal/date/datetime/tim, or none which treated as min value,
    /// otherwise error raised.
    /// and all values must be same type, except datetime will be automatically cast to date if there is date type existing,
    /// if any value is none, returns none.
    ///
    /// e.g.
    /// - ["1", "1980-01-02", None] -> error, incompatible types decimal and date.
    /// - [1, 100, "204"] -> 1, string cast to decimal.
    /// - ["1980-01-02", "1979-11-30 12:23:45", None] -> "1979-11-30", datetime downgrade to date.
    /// - ["1979-11-30 12:23:45", "12:23:45"] -> error, incompatible types date/datetime and date.
    #[restrict(none_context = true, max_param_count = 0)]
    Min,
    /// min decimal elements of vec, [only in-memory].
    /// [x.&minNum], [x.&minNum()], [&minNum(x)]
    ///
    /// - [context]: vec, none.
    /// - [none context], returns none.
    ///
    /// each value in vec should be a decimal, or can cast to decimal, or none which treated as min value,
    /// otherwise error raised.
    /// if any value is none, returns none.
    #[restrict(none_context = true, max_param_count = 0)]
    MinNum,
    /// min date of elements of vec, [only in-memory].
    /// [x.&minDate], [x.&minDate()], [&minDate(x)]
    ///
    /// - [context]: vec, none.
    /// - [none context], returns none.
    ///
    /// each value in vec should be a date/datetime, or can cast to date/datetime, or none which treated as min value,
    /// otherwise error raised.
    /// if any value is none, returns none.
    #[restrict(none_context = true, max_param_count = 0)]
    MinDate,
    /// min date time of elements of vec, [only in-memory].
    /// [x.&minDatetime], [x.&minDatetime()], [&minDatetime(x)]
    ///
    /// - [context]: vec, none.
    /// - [none context], returns none.
    ///
    /// each value in vec should be a datetime, or can cast to datetime, or none which treated as min value,
    /// otherwise error raised.
    /// if any value is none, returns none.
    #[restrict(none_context = true, max_param_count = 0)]
    MinDatetime,
    /// alias of [VariablePredefineFunctions::MinDatetime].
    /// [x.&minDt], [x.&minDt()], [&minDt(x)]
    #[restrict(none_context = true, max_param_count = 0)]
    MinDt,
    /// min time of elements of vec, [only in-memory].
    /// [x.&minTime], [x.&minTime()], [&minTime(x)]
    ///
    /// - [context]: vec, none.
    /// - [none context], returns none.
    ///
    /// each value in vec should be a time, or can cast to time, or none which treated as min value,
    /// otherwise error raised.
    /// if any value is none, returns none.
    #[restrict(none_context = true, max_param_count = 0)]
    MinTime,
    /// Retrieve value from current context, include variables and current trigger data
    /// [&cur], [&cur()]
    #[display = "&cur"]
    #[restrict(context = false, max_param_count = 0)]
    FromCurrentTriggerData,
    /// Retrieve value from previous trigger data
    /// [&old], [&old()]
    #[display = "&old"]
    #[restrict(context = false, max_param_count = 0)]
    FromPreviousTriggerData,
    // Date related functions
    /// get day difference between two dates.
    /// [x.&dayDiff(otherDate)], [&dayDiff(x, otherDate)]
    ///
    /// - [context]: date/datetime, string can cast to date/datetime.
    /// - [otherDate]: date/datetime, string can cast to date/datetime.
    #[restrict(min_param_count = 1, max_param_count = 1)]
    DayDiff,
    /// get month difference between two dates.
    /// [x.&monthDiff(otherDate)], [&monthDiff(x, otherDate)]
    ///
    /// - [context]: date/datetime, string can cast to date/datetime.
    /// - [otherDate]: date/datetime, string can cast to date/datetime.
    #[restrict(min_param_count = 1, max_param_count = 1)]
    MonthDiff,
    /// get year difference between two dates.
    /// [x.&yearDiff(otherDate)], [&yearDiff(x, otherDate)]
    ///
    /// - [context]: date/datetime, string can cast to date/datetime.
    /// - [otherDate]: date/datetime, string can cast to date/datetime.
    #[restrict(min_param_count = 1, max_param_count = 1)]
    YearDiff,
    /// move date by given days, months, years.
    /// [x.&moveDate(movement)], [&moveDate(x, movement)]
    ///
    /// - [context]: date/datetime, string can cast to date/datetime.
    /// - [movement]: string. format:
    ///   - unit: YMDhms,
    ///   - positive/negative: +/-, optional,
    ///   - if 2nd part is +/-, any number value; or
    ///     -year(Y): 4 digits year,
    ///		- month(M): 1 - 12. any value not in [1, 12] will be normalized to [1, 12],
    ///		- date(D): 1 - end of month (28/29/30/31). 99 means end of month,
    ///       otherwise any value not in [1, end of month] will be normalized to [1, end of month],
    ///		- hour(h): 0 - 23. any value not in [0, 23] will be normalized to [0, 23],
    ///		- minute(m): 0 - 59. any value not in [0, 59] will be normalized to [0, 59],
    ///		- second(s): 0 - 59. any value not in [0, 59] will be normalized to [0, 59],
    ///   - whitespaces between 3 parts are allowed, and ignored.
    /// if no time moved, then original date returned. otherwise automatically upgrade date to datetime.
    ///
    /// e.g. [date.&moveDate(Y2000M+1D-1h23m+5s-6)],
    /// if date is 1999-11-30, then result is 2000-12-29 23:04:54:
    /// - year set to 2000, now is 2000-11-30,
    /// - month plus 1, to 12, now is 2000-12-30,
    /// - day minus 1, to 29, now is 2000-12-29,
    /// - hour set to 23, now is 2000-12-29 23:00:00 (original no time, default is 00:00:00),
    /// - minute plus 5, to 5, now is 2000-12-29 23:05:00,
    /// - second minus 6, to 54, and minute minus 1. result is 2000-12-29 23:04:54.
    #[restrict(min_param_count = 1, max_param_count = 1)]
    MoveDate,
    /// format date to string by given format.
    /// [x.&dateFormat(format)], [&dateFormat(x, format)]
    ///
    /// - [context]: date/datetime, string can cast to date/datetime.
    /// - [format]: string. date format.
    ///   - 'Y': '%Y',  # 4 digits year
    ///   - 'y': '%y',  # 2 digits year
    ///   - 'M': '%m',  # 2 digits month
    ///   - 'D': '%d',  # 2 digits day of month
    ///   - 'h': '%H',  # 2 digits hour, 00 - 23
    ///   - 'H': '%I',  # 2 digits hour, 01 - 12
    ///   - 'm': '%M',  # 2 digits minute
    ///   - 's': '%S',  # 2 digits second
    ///   - 'W': '%A',  # Monday - Sunday
    ///   - 'w': '%a',  # Mon - Sun
    ///   - 'B': '%B',  # January - December
    ///   - 'b': '%b',  # Jan - Dec
    ///	  - 'p': '%p'  # AM/PM
    ///
    /// e.g. [date.&fmtDate(%Y-%M-%D)],
    /// if date is 2000-12-29, then result is 2000-12-29 00:00:00.
    #[display = "&fmtDate"]
    #[restrict(min_param_count = 1, max_param_count = 1)]
    DateFormat,
    /// get current date time.
    /// [&now], [&now()]
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
