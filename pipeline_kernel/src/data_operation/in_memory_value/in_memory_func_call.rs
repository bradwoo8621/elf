use crate::{ArcTopicDataValue, FuncDataPath, PipelineKernelErrorCode};
use bigdecimal::FromPrimitive;
use elf_base::{ErrorCode, StdR};
use elf_model::VariablePredefineFunctions;
use std::ops::Deref;
use std::sync::Arc;

pub struct InMemoryFuncCall<'a> {
    path: &'a FuncDataPath,
}

/// property
impl<'a> InMemoryFuncCall<'a> {
    pub fn path(&self) -> &'a FuncDataPath {
        &self.path
    }

    pub fn this_path(&self) -> String {
        self.path.this_path()
    }

    pub fn full_path(&self) -> String {
        self.path.full_path()
    }

    pub fn func(&self) -> &VariablePredefineFunctions {
        self.path.func()
    }
}

impl InMemoryFuncCall<'_> {
    fn resolve_replace(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        if params.len() != 2 {
            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Replace function[path={}, name={}] requires exactly 2 parameters.",
                self.path.full_path(),
                self.path.this_path()
            ));
        }
        let old_substring = self.extract_string(&params[0])?;
        let new_substring = self.extract_string(&params[1])?;
        let str_value = self.extract_string(&context)?;
        Ok(ArcTopicDataValue::arc_from(
            str_value.replace(&old_substring, &new_substring),
        ))
    }
    fn resolve_replace_first(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        if params.len() != 2 {
            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "ReplaceFirst function[path={}, name={}] requires exactly 2 parameters.",
                self.path.full_path(),
                self.path.this_path()
            ));
        }
        let old_substring = self.extract_string(&params[0])?;
        let new_substring = self.extract_string(&params[1])?;
        let str_value = self.extract_string(&context)?;
        Ok(ArcTopicDataValue::arc_from(str_value.replacen(
            &old_substring,
            &new_substring,
            1,
        )))
    }
    fn resolve_upper(
        &self,
        context: Arc<ArcTopicDataValue>,
        _params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let str_value = self.extract_string(&context)?;
        Ok(ArcTopicDataValue::arc_from(str_value.to_uppercase()))
    }
    fn resolve_lower(
        &self,
        context: Arc<ArcTopicDataValue>,
        _params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let str_value = self.extract_string(&context)?;
        Ok(ArcTopicDataValue::arc_from(str_value.to_lowercase()))
    }
    fn resolve_contains(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        if params.len() != 1 {
            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Contains function[path={}, name={}] requires exactly 1 parameter.",
                self.path.full_path(),
                self.path.this_path()
            ));
        }
        let substring = self.extract_string(&params[0])?;
        let str_value = self.extract_string(&context)?;
        Ok(ArcTopicDataValue::arc_from(str_value.contains(&substring)))
    }
    fn resolve_split(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let separator = if params.is_empty() {
            ",".to_string()
        } else if params.len() == 1 {
            self.extract_string(&params[0])?
        } else {
            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Split function[path={}, name={}] requires at most 1 parameter.",
                self.path.full_path(),
                self.path.this_path()
            ));
        };
        let str_value = self.extract_string(&context)?;
        let parts: Vec<Arc<ArcTopicDataValue>> = if separator.is_empty() {
            str_value
                .chars()
                .map(|c| ArcTopicDataValue::arc_from(c.to_string()))
                .collect()
        } else {
            str_value
                .split(&separator)
                .map(|s| ArcTopicDataValue::arc_from(s.to_string()))
                .collect()
        };
        Ok(ArcTopicDataValue::arc_from(parts))
    }
    fn resolve_concat(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let mut result = self.extract_string(&context)?;

        for param in params {
            if let ArcTopicDataValue::None = *param {
                continue;
            }
            result.push_str(&self.extract_string(&param)?);
        }
        Ok(ArcTopicDataValue::arc_from(result))
    }
    fn resolve_concat_with(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        if params.is_empty() {
            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "ConcatWith function[path={}, name={}] requires at least 2 parameters.",
                self.path.full_path(),
                self.path.this_path()
            ));
        }
        let separator = self.extract_string(&params[0])?;
        let mut result = self.extract_string(&context)?;

        for param in params.iter().skip(1) {
            if let ArcTopicDataValue::None = *param {
                continue;
            }
            result.push_str(&separator);
            result.push_str(&self.extract_string(param)?);
        }
        Ok(ArcTopicDataValue::arc_from(result))
    }
    fn resolve_join(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let separator = if params.is_empty() {
            ",".to_string()
        } else if params.len() == 1 {
            self.extract_string(&params[0])?
        } else {
            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Join function[path={}, name={}] requires at most 1 parameter.",
                self.path.full_path(),
                self.path.this_path()
            ));
        };
        let vec_value = self.extract_vec(&context)?;
        let strings: Vec<String> = vec_value
            .iter()
            .filter_map(|item| {
                if let ArcTopicDataValue::None = **item {
                    None
                } else {
                    self.extract_string(item).ok()
                }
            })
            .collect();
        Ok(ArcTopicDataValue::arc_from(strings.join(&separator)))
    }
    fn resolve_distinct(
        &self,
        context: Arc<ArcTopicDataValue>,
        _params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let vec_value = self.extract_vec(&context)?;
        let mut seen = std::collections::HashSet::new();
        let distinct: Vec<Arc<ArcTopicDataValue>> = vec_value
            .iter()
            .filter(|item| {
                if let ArcTopicDataValue::None = ***item {
                    false
                } else {
                    let display = format!("{}", item);
                    seen.insert(display)
                }
            })
            .cloned()
            .collect();
        Ok(ArcTopicDataValue::arc_from(distinct))
    }
    fn resolve_sum(
        &self,
        context: Arc<ArcTopicDataValue>,
        _params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        context.sum_of_vec(
            || {
                PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                    "Failed to parse decimal for sum function[path={}, name={}].",
                    self.path.full_path(),
                    self.path.this_path()
                ))
            },
            || {
                PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                    "Sum function[path={}, name={}] only supports vec.",
                    self.path.full_path(),
                    self.path.this_path()
                ))
            },
        )
    }
    fn resolve_avg(
        &self,
        context: Arc<ArcTopicDataValue>,
        _params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        context.avg_of_vec(
            || {
                PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                    "Failed to parse decimal for avg function[path={}, name={}].",
                    self.path.full_path(),
                    self.path.this_path()
                ))
            },
            || {
                PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                    "Avg function[path={}, name={}] only supports vec.",
                    self.path.full_path(),
                    self.path.this_path()
                ))
            },
        )
    }
    fn resolve_max(
        &self,
        context: Arc<ArcTopicDataValue>,
        _params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        context.max_of_vec(|| {
            PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Max function[path={}, name={}] only supports vec.",
                self.path.full_path(),
                self.path.this_path()
            ))
        })
    }
    fn resolve_max_num(
        &self,
        context: Arc<ArcTopicDataValue>,
        _params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        context.max_decimal_of_vec(|| {
            PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "MaxNum function[path={}, name={}] only supports vec.",
                self.path.full_path(),
                self.path.this_path()
            ))
        })
    }
    fn resolve_max_date(
        &self,
        context: Arc<ArcTopicDataValue>,
        _params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        context.max_date_of_vec(|| {
            PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "MaxDate function[path={}, name={}] only supports vec.",
                self.path.full_path(),
                self.path.this_path()
            ))
        })
    }
    fn resolve_max_datetime(
        &self,
        context: Arc<ArcTopicDataValue>,
        _params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        context.max_datetime_of_vec(|| {
            PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "MaxDatetime function[path={}, name={}] only supports vec.",
                self.path.full_path(),
                self.path.this_path()
            ))
        })
    }
    fn resolve_max_time(
        &self,
        context: Arc<ArcTopicDataValue>,
        _params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        context.max_time_of_vec(|| {
            PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "MaxTime function[path={}, name={}] only supports vec.",
                self.path.full_path(),
                self.path.this_path()
            ))
        })
    }
    fn resolve_min(
        &self,
        context: Arc<ArcTopicDataValue>,
        _params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        context.min_of_vec(|| {
            PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Min function[path={}, name={}] only supports vec.",
                self.path.full_path(),
                self.path.this_path()
            ))
        })
    }
    fn resolve_min_num(
        &self,
        context: Arc<ArcTopicDataValue>,
        _params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        context.min_decimal_of_vec(|| {
            PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "MinNum function[path={}, name={}] only supports vec.",
                self.path.full_path(),
                self.path.this_path()
            ))
        })
    }
    fn resolve_min_date(
        &self,
        context: Arc<ArcTopicDataValue>,
        _params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        context.min_date_of_vec(|| {
            PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "MinDate function[path={}, name={}] only supports vec.",
                self.path.full_path(),
                self.path.this_path()
            ))
        })
    }
    fn resolve_min_datetime(
        &self,
        context: Arc<ArcTopicDataValue>,
        _params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        context.min_datetime_of_vec(|| {
            PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "MinDatetime function[path={}, name={}] only supports vec.",
                self.path.full_path(),
                self.path.this_path()
            ))
        })
    }
    fn resolve_min_time(
        &self,
        context: Arc<ArcTopicDataValue>,
        _params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        context.min_time_of_vec(|| {
            PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "MinTime function[path={}, name={}] only supports vec.",
                self.path.full_path(),
                self.path.this_path()
            ))
        })
    }
    fn resolve_day_diff(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        if params.len() != 1 {
            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "DayDiff function[path={}, name={}] requires exactly 1 parameter.",
                self.path.full_path(),
                self.path.this_path()
            ));
        }
        let start_date = self.extract_datetime(&context)?;
        let end_date = self.extract_datetime(&params[0])?;
        let diff = (end_date.date() - start_date.date()).num_days();
        Ok(ArcTopicDataValue::arc_from(
            bigdecimal::BigDecimal::from_i64(diff).unwrap(),
        ))
    }
    fn resolve_month_diff(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        if params.len() != 1 {
            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "MonthDiff function[path={}, name={}] requires exactly 1 parameter.",
                self.path.full_path(),
                self.path.this_path()
            ));
        }
        let start_date = self.extract_datetime(&context)?;
        let end_date = self.extract_datetime(&params[0])?;
        let months = (end_date.year() - start_date.year()) * 12
            + (end_date.month() - start_date.month()) as i32;
        Ok(ArcTopicDataValue::arc_from(
            bigdecimal::BigDecimal::from_i32(months).unwrap(),
        ))
    }
    fn resolve_year_diff(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        if params.len() != 1 {
            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "YearDiff function[path={}, name={}] requires exactly 1 parameter.",
                self.path.full_path(),
                self.path.this_path()
            ));
        }
        let start_date = self.extract_datetime(&context)?;
        let end_date = self.extract_datetime(&params[0])?;
        let years = end_date.year() - start_date.year();
        Ok(ArcTopicDataValue::arc_from(
            bigdecimal::BigDecimal::from_i32(years).unwrap(),
        ))
    }
    fn resolve_move_date(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        if params.len() != 1 {
            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "MoveDate function[path={}, name={}] requires exactly 1 parameter.",
                self.path.full_path(),
                self.path.this_path()
            ));
        }
        let movement_str = self.extract_string(&params[0])?;
        let mut date = self.extract_datetime(&context)?;
        // Parse movement string (simplified implementation)
        // Format: [+/-]Y[year]M[month]D[day]h[hour]m[minute]s[second]
        let mut chars = movement_str.chars().peekable();
        let mut sign = 1;

        if let Some(&'+') = chars.peek() {
            chars.next();
        } else if let Some(&'-') = chars.peek() {
            chars.next();
            sign = -1;
        }
        while let Some(c) = chars.next() {
            let mut num_str = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_digit() {
                    num_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            let num: i64 = if num_str.is_empty() {
                1
            } else {
                num_str.parse().unwrap_or(0)
            } * sign;

            match c {
                'Y' => date = date.with_year(date.year() + num as i32).unwrap_or(date),
                'M' => {
                    let new_month = date.month() as i32 + num;
                    if new_month > 0 {
                        date = date.with_month((new_month - 1) % 12 + 1).unwrap_or(date);
                        date = date
                            .with_year(date.year() + (new_month - 1) / 12)
                            .unwrap_or(date);
                    }
                }
                'D' => date = date + chrono::Duration::days(num),
                'h' => date = date + chrono::Duration::hours(num),
                'm' => date = date + chrono::Duration::minutes(num),
                's' => date = date + chrono::Duration::seconds(num),
                _ => {}
            }
        }
        Ok(ArcTopicDataValue::arc_from(date))
    }
    fn resolve_date_format(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        if params.len() != 1 {
            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "DateFormat function[path={}, name={}] requires exactly 1 parameter.",
                self.path.full_path(),
                self.path.this_path()
            ));
        }
        let format_str = self.extract_string(&params[0])?;
        let date = self.extract_datetime(&context)?;

        // Convert custom format to chrono format
        let chrono_format = format_str
            .replace('Y', "%Y")
            .replace('y', "%y")
            .replace('M', "%m")
            .replace('D', "%d")
            .replace('h', "%H")
            .replace('H', "%I")
            .replace('m', "%M")
            .replace('s', "%S")
            .replace('W', "%A")
            .replace('w', "%a")
            .replace('B', "%B")
            .replace('b', "%b")
            .replace('p', "%p");
        Ok(ArcTopicDataValue::arc_from(
            date.format(&chrono_format).to_string(),
        ))
    }
    // Helper methods for extracting values
    fn extract_string(&self, value: &Arc<ArcTopicDataValue>) -> StdR<String> {
        match value {
            ArcTopicDataValue::Str(s) => Ok(s.to_string()),
            ArcTopicDataValue::Num(n) => Ok(elf_base::StringConverter::to_string(n)),
            ArcTopicDataValue::Bool(b) => Ok(b.to_string()),
            ArcTopicDataValue::DateTime(dt) => Ok(dt.to_string()),
            ArcTopicDataValue::Date(d) => Ok(d.to_string()),
            ArcTopicDataValue::Time(t) => Ok(t.to_string()),
            ArcTopicDataValue::None => Ok(String::new()),
            _ => PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Cannot convert value to string for function[path={}, name={}].",
                self.path.full_path(),
                self.path.this_path()
            )),
        }
    }
    fn extract_decimal(
        &self,
        value: &Arc<ArcTopicDataValue>,
    ) -> StdR<Option<bigdecimal::BigDecimal>> {
        match value.deref() {
            ArcTopicDataValue::Num(n) => Ok(Some(n.as_ref().clone())),
            ArcTopicDataValue::Str(s) => {
                if s.trim().is_empty() {
                    Ok(None)
                } else {
                    s.parse().map(Some).map_err(|_| {
                        PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                            "Cannot parse string '{}' as decimal for function[path={}, name={}].",
                            s,
                            self.path.full_path(),
                            self.path.this_path()
                        ))
                    })
                }
            }
            ArcTopicDataValue::None => Ok(None),
            _ => PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Cannot convert value to decimal for function[path={}, name={}].",
                self.path.full_path(),
                self.path.this_path()
            )),
        }
    }
    fn extract_datetime(&self, value: &Arc<ArcTopicDataValue>) -> StdR<chrono::NaiveDateTime> {
        match value.deref() {
            ArcTopicDataValue::DateTime(dt) => Ok(**dt),
            ArcTopicDataValue::Date(d) => Ok(d.and_hms_opt(0, 0, 0).unwrap()),
            ArcTopicDataValue::Str(s) => s.parse().map_err(|_| {
                PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                    "Cannot parse string '{}' as datetime for function[path={}, name={}].",
                    s,
                    self.path.full_path(),
                    self.path.this_path()
                ))
            }),
            _ => PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Cannot convert value to datetime for function[path={}, name={}].",
                self.path.full_path(),
                self.path.this_path()
            )),
        }
    }
    fn extract_vec(
        &self,
        value: &Arc<ArcTopicDataValue>,
    ) -> StdR<&Arc<Vec<Arc<ArcTopicDataValue>>>> {
        match value.deref() {
            ArcTopicDataValue::Vec(vec) => Ok(vec),
            _ => PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Expected vec value for function[path={}, name={}].",
                self.path.full_path(),
                self.path.this_path()
            )),
        }
    }
}

/// compute
impl<'a> InMemoryFuncCall<'a> {
    fn do_compute(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        match self.path.func() {
            VariablePredefineFunctions::NextSeq => self.context_disallowed(),
            VariablePredefineFunctions::Count => self.resolve_count_of_vec_or_map(context),
            VariablePredefineFunctions::Length | VariablePredefineFunctions::Len => {
                self.resolve_length_of_str_or_num(context)
            }
            VariablePredefineFunctions::Slice | VariablePredefineFunctions::Substr => {
                self.resolve_slice_of_str(context, params)
            }
            VariablePredefineFunctions::Find | VariablePredefineFunctions::Index => {
                self.resolve_find_of_str(context, params)
            }
            VariablePredefineFunctions::StartsWith | VariablePredefineFunctions::Startswith => {
                self.resolve_starts_with_of_str(context, params)
            }
            VariablePredefineFunctions::EndsWith | VariablePredefineFunctions::Endswith => {
                self.resolve_ends_with_of_str(context, params)
            }
            VariablePredefineFunctions::Strip | VariablePredefineFunctions::Trim => {
                self.resolve_trim_of_str(context, params)
            }
            VariablePredefineFunctions::Replace => self.resolve_replace(context, params),
            VariablePredefineFunctions::ReplaceFirst => self.resolve_replace_first(context, params),
            VariablePredefineFunctions::Upper => self.resolve_upper(context, params),
            VariablePredefineFunctions::Lower => self.resolve_lower(context, params),
            VariablePredefineFunctions::Contains => self.resolve_contains(context, params),
            VariablePredefineFunctions::Split => self.resolve_split(context, params),
            VariablePredefineFunctions::Concat => self.resolve_concat(context, params),
            VariablePredefineFunctions::ConcatWith => self.resolve_concat_with(context, params),
            VariablePredefineFunctions::Join => self.resolve_join(context, params),
            VariablePredefineFunctions::Distinct => self.resolve_distinct(context, params),
            VariablePredefineFunctions::Sum => self.resolve_sum(context, params),
            VariablePredefineFunctions::Avg => self.resolve_avg(context, params),
            VariablePredefineFunctions::Max => self.resolve_max(context, params),
            VariablePredefineFunctions::MaxNum => self.resolve_max_num(context, params),
            VariablePredefineFunctions::MaxDate => self.resolve_max_date(context, params),
            VariablePredefineFunctions::MaxDatetime => self.resolve_max_datetime(context, params),
            VariablePredefineFunctions::MaxDt => self.resolve_max_datetime(context, params),
            VariablePredefineFunctions::MaxTime => self.resolve_max_time(context, params),
            VariablePredefineFunctions::Min => self.resolve_min(context, params),
            VariablePredefineFunctions::MinNum => self.resolve_min_num(context, params),
            VariablePredefineFunctions::MinDate => self.resolve_min_date(context, params),
            VariablePredefineFunctions::MinDatetime => self.resolve_min_datetime(context, params),
            VariablePredefineFunctions::MinDt => self.resolve_min_datetime(context, params),
            VariablePredefineFunctions::MinTime => self.resolve_min_time(context, params),
            VariablePredefineFunctions::FromCurrentTriggerData => self.context_disallowed(),
            VariablePredefineFunctions::FromPreviousTriggerData => self.context_disallowed(),
            VariablePredefineFunctions::DayDiff => self.resolve_day_diff(context, params),
            VariablePredefineFunctions::MonthDiff => self.resolve_month_diff(context, params),
            VariablePredefineFunctions::YearDiff => self.resolve_year_diff(context, params),
            VariablePredefineFunctions::MoveDate => self.resolve_move_date(context, params),
            VariablePredefineFunctions::DateFormat => self.resolve_date_format(context, params),
            VariablePredefineFunctions::Now => self.context_disallowed(),
        }
    }

    /// compute the function value by given context and parameters.
    /// will check the min/max parameter count
    pub fn compute(
        path: &'a FuncDataPath,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let param_count = params.len();
        let func = path.func();
        let min_param_count = func.min_param_count();
        if param_count < min_param_count {
            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Function[path={}, name={}] has no enough parameters, at least {} are required, but only {} are currently provided.",
                path.full_path(),
                func, min_param_count, param_count
            ));
        }
        if let Some(max_param_count) = func.max_param_count() {
            if param_count > max_param_count {
                return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                    "Function[path={}, name={}] has too many parameters, at most {} are accepted, but {} are currently provided.",
                    path.full_path(),
                    func, max_param_count, param_count
                ));
            }
        }

        Self { path }.do_compute(context, params)
    }
}
