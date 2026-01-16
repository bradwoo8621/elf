// impl InMemoryFuncCall<'_> {
// 	fn resolve_distinct(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		_params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		let vec_value = self.extract_vec(&context)?;
// 		let mut seen = std::collections::HashSet::new();
// 		let distinct: Vec<Arc<ArcTopicDataValue>> = vec_value
// 			.iter()
// 			.filter(|item| {
// 				if let ArcTopicDataValue::None = ***item {
// 					false
// 				} else {
// 					let display = format!("{}", item);
// 					seen.insert(display)
// 				}
// 			})
// 			.cloned()
// 			.collect();
// 		Ok(ArcTopicDataValue::arc_from(distinct))
// 	}
// 	fn resolve_sum(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		_params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		context.sum_of_vec(
// 			|| {
// 				PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 					"Failed to parse decimal for sum function[path={}, name={}].",
// 					self.path.full_path(),
// 					self.path.this_path()
// 				))
// 			},
// 			|| {
// 				PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 					"Sum function[path={}, name={}] only supports vec.",
// 					self.path.full_path(),
// 					self.path.this_path()
// 				))
// 			},
// 		)
// 	}
// 	fn resolve_avg(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		_params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		context.avg_of_vec(
// 			|| {
// 				PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 					"Failed to parse decimal for avg function[path={}, name={}].",
// 					self.path.full_path(),
// 					self.path.this_path()
// 				))
// 			},
// 			|| {
// 				PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 					"Avg function[path={}, name={}] only supports vec.",
// 					self.path.full_path(),
// 					self.path.this_path()
// 				))
// 			},
// 		)
// 	}
// 	fn resolve_max(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		_params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		context.max_of_vec(|| {
// 			PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"Max function[path={}, name={}] only supports vec.",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			))
// 		})
// 	}
// 	fn resolve_max_num(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		_params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		context.max_decimal_of_vec(|| {
// 			PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"MaxNum function[path={}, name={}] only supports vec.",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			))
// 		})
// 	}
// 	fn resolve_max_date(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		_params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		context.max_date_of_vec(|| {
// 			PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"MaxDate function[path={}, name={}] only supports vec.",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			))
// 		})
// 	}
// 	fn resolve_max_datetime(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		_params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		context.max_datetime_of_vec(|| {
// 			PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"MaxDatetime function[path={}, name={}] only supports vec.",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			))
// 		})
// 	}
// 	fn resolve_max_time(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		_params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		context.max_time_of_vec(|| {
// 			PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"MaxTime function[path={}, name={}] only supports vec.",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			))
// 		})
// 	}
// 	fn resolve_min(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		_params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		context.min_of_vec(|| {
// 			PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"Min function[path={}, name={}] only supports vec.",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			))
// 		})
// 	}
// 	fn resolve_min_num(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		_params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		context.min_decimal_of_vec(|| {
// 			PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"MinNum function[path={}, name={}] only supports vec.",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			))
// 		})
// 	}
// 	fn resolve_min_date(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		_params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		context.min_date_of_vec(|| {
// 			PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"MinDate function[path={}, name={}] only supports vec.",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			))
// 		})
// 	}
// 	fn resolve_min_datetime(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		_params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		context.min_datetime_of_vec(|| {
// 			PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"MinDatetime function[path={}, name={}] only supports vec.",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			))
// 		})
// 	}
// 	fn resolve_min_time(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		_params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		context.min_time_of_vec(|| {
// 			PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"MinTime function[path={}, name={}] only supports vec.",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			))
// 		})
// 	}
// 	fn resolve_day_diff(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		if params.len() != 1 {
// 			return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"DayDiff function[path={}, name={}] requires exactly 1 parameter.",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			));
// 		}
// 		let start_date = self.extract_datetime(&context)?;
// 		let end_date = self.extract_datetime(&params[0])?;
// 		let diff = (end_date.date() - start_date.date()).num_days();
// 		Ok(ArcTopicDataValue::arc_from(
// 			bigdecimal::BigDecimal::from_i64(diff).unwrap(),
// 		))
// 	}
// 	fn resolve_month_diff(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		if params.len() != 1 {
// 			return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"MonthDiff function[path={}, name={}] requires exactly 1 parameter.",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			));
// 		}
// 		let start_date = self.extract_datetime(&context)?;
// 		let end_date = self.extract_datetime(&params[0])?;
// 		let months = (end_date.year() - start_date.year()) * 12
// 			+ (end_date.month() - start_date.month()) as i32;
// 		Ok(ArcTopicDataValue::arc_from(
// 			bigdecimal::BigDecimal::from_i32(months).unwrap(),
// 		))
// 	}
// 	fn resolve_year_diff(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		if params.len() != 1 {
// 			return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"YearDiff function[path={}, name={}] requires exactly 1 parameter.",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			));
// 		}
// 		let start_date = self.extract_datetime(&context)?;
// 		let end_date = self.extract_datetime(&params[0])?;
// 		let years = end_date.year() - start_date.year();
// 		Ok(ArcTopicDataValue::arc_from(
// 			bigdecimal::BigDecimal::from_i32(years).unwrap(),
// 		))
// 	}
// 	fn resolve_move_date(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		if params.len() != 1 {
// 			return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"MoveDate function[path={}, name={}] requires exactly 1 parameter.",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			));
// 		}
// 		let movement_str = self.extract_string(&params[0])?;
// 		let mut date = self.extract_datetime(&context)?;
// 		// Parse movement string (simplified implementation)
// 		// Format: [+/-]Y[year]M[month]D[day]h[hour]m[minute]s[second]
// 		let mut chars = movement_str.chars().peekable();
// 		let mut sign = 1;
//
// 		if let Some(&'+') = chars.peek() {
// 			chars.next();
// 		} else if let Some(&'-') = chars.peek() {
// 			chars.next();
// 			sign = -1;
// 		}
// 		while let Some(c) = chars.next() {
// 			let mut num_str = String::new();
// 			while let Some(&ch) = chars.peek() {
// 				if ch.is_ascii_digit() {
// 					num_str.push(chars.next().unwrap());
// 				} else {
// 					break;
// 				}
// 			}
//
// 			let num: i64 = if num_str.is_empty() {
// 				1
// 			} else {
// 				num_str.parse().unwrap_or(0)
// 			} * sign;
//
// 			match c {
// 				'Y' => date = date.with_year(date.year() + num as i32).unwrap_or(date),
// 				'M' => {
// 					let new_month = date.month() as i32 + num;
// 					if new_month > 0 {
// 						date = date.with_month((new_month - 1) % 12 + 1).unwrap_or(date);
// 						date = date
// 							.with_year(date.year() + (new_month - 1) / 12)
// 							.unwrap_or(date);
// 					}
// 				}
// 				'D' => date = date + chrono::Duration::days(num),
// 				'h' => date = date + chrono::Duration::hours(num),
// 				'm' => date = date + chrono::Duration::minutes(num),
// 				's' => date = date + chrono::Duration::seconds(num),
// 				_ => {}
// 			}
// 		}
// 		Ok(ArcTopicDataValue::arc_from(date))
// 	}
// 	fn resolve_date_format(
// 		&self,
// 		context: Arc<ArcTopicDataValue>,
// 		params: Vec<Arc<ArcTopicDataValue>>,
// 	) -> StdR<Arc<ArcTopicDataValue>> {
// 		if params.len() != 1 {
// 			return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"DateFormat function[path={}, name={}] requires exactly 1 parameter.",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			));
// 		}
// 		let format_str = self.extract_string(&params[0])?;
// 		let date = self.extract_datetime(&context)?;
//
// 		// Convert custom format to chrono format
// 		let chrono_format = format_str
// 			.replace('Y', "%Y")
// 			.replace('y', "%y")
// 			.replace('M', "%m")
// 			.replace('D', "%d")
// 			.replace('h', "%H")
// 			.replace('H', "%I")
// 			.replace('m', "%M")
// 			.replace('s', "%S")
// 			.replace('W', "%A")
// 			.replace('w', "%a")
// 			.replace('B', "%B")
// 			.replace('b', "%b")
// 			.replace('p', "%p");
// 		Ok(ArcTopicDataValue::arc_from(
// 			date.format(&chrono_format).to_string(),
// 		))
// 	}
// 	// Helper methods for extracting values
// 	fn extract_string(&self, value: &Arc<ArcTopicDataValue>) -> StdR<String> {
// 		match value {
// 			ArcTopicDataValue::Str(s) => Ok(s.to_string()),
// 			ArcTopicDataValue::Num(n) => Ok(elf_base::StringConverter::to_string(n)),
// 			ArcTopicDataValue::Bool(b) => Ok(b.to_string()),
// 			ArcTopicDataValue::DateTime(dt) => Ok(dt.to_string()),
// 			ArcTopicDataValue::Date(d) => Ok(d.to_string()),
// 			ArcTopicDataValue::Time(t) => Ok(t.to_string()),
// 			ArcTopicDataValue::None => Ok(String::new()),
// 			_ => PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"Cannot convert value to string for function[path={}, name={}].",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			)),
// 		}
// 	}
// 	fn extract_decimal(
// 		&self,
// 		value: &Arc<ArcTopicDataValue>,
// 	) -> StdR<Option<bigdecimal::BigDecimal>> {
// 		match value.deref() {
// 			ArcTopicDataValue::Num(n) => Ok(Some(n.as_ref().clone())),
// 			ArcTopicDataValue::Str(s) => {
// 				if s.trim().is_empty() {
// 					Ok(None)
// 				} else {
// 					s.parse().map(Some).map_err(|_| {
// 						PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 							"Cannot parse string '{}' as decimal for function[path={}, name={}].",
// 							s,
// 							self.path.full_path(),
// 							self.path.this_path()
// 						))
// 					})
// 				}
// 			}
// 			ArcTopicDataValue::None => Ok(None),
// 			_ => PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"Cannot convert value to decimal for function[path={}, name={}].",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			)),
// 		}
// 	}
// 	fn extract_datetime(&self, value: &Arc<ArcTopicDataValue>) -> StdR<chrono::NaiveDateTime> {
// 		match value.deref() {
// 			ArcTopicDataValue::DateTime(dt) => Ok(**dt),
// 			ArcTopicDataValue::Date(d) => Ok(d.and_hms_opt(0, 0, 0).unwrap()),
// 			ArcTopicDataValue::Str(s) => s.parse().map_err(|_| {
// 				PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 					"Cannot parse string '{}' as datetime for function[path={}, name={}].",
// 					s,
// 					self.path.full_path(),
// 					self.path.this_path()
// 				))
// 			}),
// 			_ => PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"Cannot convert value to datetime for function[path={}, name={}].",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			)),
// 		}
// 	}
// 	fn extract_vec(
// 		&self,
// 		value: &Arc<ArcTopicDataValue>,
// 	) -> StdR<&Arc<Vec<Arc<ArcTopicDataValue>>>> {
// 		match value.deref() {
// 			ArcTopicDataValue::Vec(vec) => Ok(vec),
// 			_ => PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
// 				"Expected vec value for function[path={}, name={}].",
// 				self.path.full_path(),
// 				self.path.this_path()
// 			)),
// 		}
// 	}
// }
