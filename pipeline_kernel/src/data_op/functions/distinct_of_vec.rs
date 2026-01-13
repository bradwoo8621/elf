use crate::ArcTopicDataValue;
use elf_base::StdR;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

impl ArcTopicDataValue {
    /// distinct elements, can be applied on vec only
    /// for each element in vec,
    /// - str, decimal, datetime, date, time -> with the same type and value will be distinct,
    /// - bool -> maximum 2: true and false,
    /// - none -> maximum 1
    /// - vec, map -> cannot be removed as duplicates and are always added to the result.
    pub fn distinct_of_vec<NotSupport>(
        &self,
        not_support: NotSupport,
    ) -> StdR<Arc<ArcTopicDataValue>>
    where
        NotSupport: FnOnce() -> StdR<Arc<ArcTopicDataValue>>,
    {
        match self {
            Self::Vec(vec) => {
                let mut distinct_values: Vec<Arc<Self>> = vec![];

                let mut none_added = false;
                let mut true_added = false;
                let mut false_added = false;
                let mut string_values = HashMap::new();
                let mut decimal_values = HashMap::new();
                let mut datetime_values = HashMap::new();
                let mut date_values = HashMap::new();
                let mut time_values = HashMap::new();

                vec.iter().for_each(|value| {
                    let should_add = match value.deref() {
                        Self::Str(str) => {
                            if !string_values.contains_key(str) {
                                string_values.insert(str.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        Self::Num(decimal) => {
                            if !decimal_values.contains_key(decimal) {
                                decimal_values.insert(decimal.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        Self::Bool(bool) => {
                            if *bool && !true_added {
                                true_added = true;
                                true
                            } else if !*bool && !false_added {
                                false_added = true;
                                true
                            } else {
                                false
                            }
                        }
                        Self::DateTime(datetime) => {
                            if !datetime_values.contains_key(datetime) {
                                datetime_values.insert(datetime.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        Self::Date(date) => {
                            if !date_values.contains_key(date) {
                                date_values.insert(date.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        Self::Time(time) => {
                            if !time_values.contains_key(time) {
                                time_values.insert(time.clone(), 1);
                                true
                            } else {
                                false
                            }
                        }
                        Self::Vec(_) => true,
                        Self::Map(_) => true,
                        Self::None => {
                            if !none_added {
                                none_added = true;
                                true
                            } else {
                                false
                            }
                        }
                    };
                    if should_add {
                        distinct_values.push(value.clone());
                    }
                });

                Ok(Arc::new(Self::Vec(Arc::new(distinct_values))))
            }
            _ => not_support(),
        }
    }
}
