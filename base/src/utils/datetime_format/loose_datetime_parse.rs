use chrono::format::ParseErrorKind::{Impossible, NotEnough, OutOfRange};
use chrono::format::{parse, ParseErrorKind, Parsed, StrftimeItems};
use chrono::{
    DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, ParseError, TimeDelta, Timelike,
};

pub struct LooseDateTimeParser;

pub struct LooseDateTimeParseError(ParseErrorKind);

impl LooseDateTimeParser {
    fn transform_error(error: ParseError) -> LooseDateTimeParseError {
        let kind = error.kind();
        LooseDateTimeParseError(kind)
    }

    /// Original rules:
    /// - Hour, minute. (second and nanosecond assumed to be 0)
    /// - Hour, minute, second. (nanosecond assumed to be 0)
    /// - Hour, minute, second, nanosecond.
    ///
    /// rewritten based on [Parsed::to_naive_time], allowing the time part to be missing (filled with 0),
    /// but adding the following validations:
    /// - No value of fields, all fields assumed to be 0,
    /// - Hour. (minute, second and nanosecond assumed to be 0)
    fn get_naive_time(parsed: &Parsed) -> Result<NaiveTime, LooseDateTimeParseError> {
        let mut has_hour = true;
        let hour_div_12 = match parsed.hour_div_12 {
            Some(v @ 0..=1) => v,
            Some(_) => return Err(LooseDateTimeParseError(OutOfRange)),
            None => {
                has_hour = false;
                0
            }
        };
        let hour_mod_12 = match parsed.hour_mod_12 {
            Some(v @ 0..=11) => v,
            Some(_) => return Err(LooseDateTimeParseError(OutOfRange)),
            None => {
                has_hour = false;
                0
            }
        };
        let hour = hour_div_12 * 12 + hour_mod_12;

        let mut has_minute = true;
        let minute = match parsed.minute {
            Some(v @ 0..=59) => v,
            Some(_) => return Err(LooseDateTimeParseError(OutOfRange)),
            None => {
                has_minute = false;
                0
            }
        };

        let has_second = parsed.second.is_some();
        // we allow omitting seconds or nanoseconds, but they should be in the range.
        let (second, mut nano) = match parsed.second.unwrap_or(0) {
            v @ 0..=59 => (v, 0),
            60 => (59, 1_000_000_000),
            _ => return Err(LooseDateTimeParseError(OutOfRange)),
        };
        nano += match parsed.nanosecond {
            Some(v @ 0..=999_999_999) if has_second => v,
            // second is missing
            Some(0..=999_999_999) => {
                return Err(LooseDateTimeParseError(NotEnough));
            }
            Some(_) => return Err(LooseDateTimeParseError(OutOfRange)),
            None => 0,
        };

        if has_minute && !has_hour {
            Err(LooseDateTimeParseError(OutOfRange))
        } else if has_second && !has_minute {
            Err(LooseDateTimeParseError(OutOfRange))
        } else {
            NaiveTime::from_hms_nano_opt(hour, minute, second, nano)
                .ok_or(LooseDateTimeParseError(OutOfRange))
        }
    }

    /// copy from [Parsed::to_naive_datetime_with_offset].
    /// the logic remains exactly the same, except that the error type and the date-time are passed in.
    fn build_datetime_with_offset(
        parsed: &Parsed,
        date: Result<NaiveDate, LooseDateTimeParseError>,
        time: Result<NaiveTime, LooseDateTimeParseError>,
        offset: i32,
    ) -> Result<NaiveDateTime, LooseDateTimeParseError> {
        use LooseDateTimeParseError as PE;

        match (date, time, parsed.timestamp) {
            (Ok(date), Ok(time), _) => {
                let datetime = date.and_time(time);

                // verify the timestamp field if any
                // the following is safe, `timestamp` is very limited in range
                let timestamp = datetime.and_utc().timestamp() - i64::from(offset);
                if let Some(given_timestamp) = parsed.timestamp {
                    // if `datetime` represents a leap second, it might be off by one second.
                    if given_timestamp != timestamp
                        && !(datetime.nanosecond() >= 1_000_000_000
                            && given_timestamp == timestamp + 1)
                    {
                        return Err(LooseDateTimeParseError(Impossible));
                    }
                }

                Ok(datetime)
            }
            // if date and time is problematic already, there is no point proceeding.
            // we at least try to give a correct error though.
            (Err(PE(OutOfRange)), _, Some(_)) => Err(LooseDateTimeParseError(OutOfRange)),
            (_, Err(PE(OutOfRange)), Some(_)) => Err(LooseDateTimeParseError(OutOfRange)),
            (Err(PE(Impossible)), _, Some(_)) => Err(LooseDateTimeParseError(Impossible)),
            (_, Err(PE(Impossible)), Some(_)) => Err(LooseDateTimeParseError(Impossible)),
            (_, _, Some(timestamp)) => {
                // reconstruct date and time fields from timestamp
                let ts = timestamp
                    .checked_add(i64::from(offset))
                    .ok_or(LooseDateTimeParseError(OutOfRange))?;
                let mut datetime = DateTime::from_timestamp_secs(ts)
                    .ok_or(LooseDateTimeParseError(OutOfRange))?
                    .naive_utc();

                // fill year, ordinal, hour, minute and second fields from timestamp.
                // if existing fields are consistent, this will allow the full date/time reconstruction.
                let mut parsed = parsed.clone();
                if parsed.second == Some(60) {
                    // `datetime.second()` cannot be 60, so this is the only case for a leap second.
                    match datetime.second() {
                        // it's okay, just do not try to overwrite the existing field.
                        59 => {}
                        // `datetime` is known to be off by one second.
                        0 => {
                            datetime -= TimeDelta::try_seconds(1).unwrap();
                        }
                        // otherwise it is impossible.
                        _ => return Err(LooseDateTimeParseError(Impossible)),
                    }
                // ...and we have the correct candidates for other fields.
                } else {
                    parsed
                        .set_second(i64::from(datetime.second()))
                        .or_else(|e| Err(Self::transform_error(e)))?;
                }
                parsed
                    .set_year(i64::from(datetime.year()))
                    .or_else(|e| Err(Self::transform_error(e)))?;
                parsed
                    .set_ordinal(i64::from(datetime.ordinal()))
                    .or_else(|e| Err(Self::transform_error(e)))?; // more efficient than ymd
                parsed
                    .set_hour(i64::from(datetime.hour()))
                    .or_else(|e| Err(Self::transform_error(e)))?;
                parsed
                    .set_minute(i64::from(datetime.minute()))
                    .or_else(|e| Err(Self::transform_error(e)))?;

                // validate other fields (e.g. week) and return
                let date = parsed
                    .to_naive_date()
                    .or_else(|e| Err(Self::transform_error(e)))?;
                let time = parsed
                    .to_naive_time()
                    .or_else(|e| Err(Self::transform_error(e)))?;
                Ok(date.and_time(time))
            }
            (Err(e), _, None) => Err(e),
            (_, Err(e), None) => Err(e),
        }
    }

    pub fn parse(str: &String, support: &String) -> Result<NaiveDateTime, LooseDateTimeParseError> {
        let mut parsed = Parsed::new();
        match parse(&mut parsed, str.as_str(), StrftimeItems::new(&support)) {
            Ok(_) => {
                let date = parsed
                    .to_naive_date()
                    .or_else(|e| Err(Self::transform_error(e)));
                let time = Self::get_naive_time(&parsed);
                Self::build_datetime_with_offset(&parsed, date, time, 0)
            }
            Err(e) => Err(Self::transform_error(e)),
        }
    }
}
