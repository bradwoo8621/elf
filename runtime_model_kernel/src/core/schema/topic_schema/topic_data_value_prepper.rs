use crate::{
	ArcFactor, ArcTopic, FakeTopicSchemaFactor, HierarchyAid, RuntimeModelKernelErrorCode,
	SimpleTopicSchemaFactor, TopicSchemaFactor, TopicSchemaFactors, TriedTDV, TriedTopicDataValue,
	VecOrMapTopicSchemaFactor,
};
use bigdecimal::BigDecimal;
use chrono::NaiveTime;
use elf_base::{DateTimeUtils, ErrorCode, StdR, VoidR};
use elf_model::{FactorType, TopicData, TopicDataValue};
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::sync::Arc;

pub struct TopicSchemaFactorValuePrepper {
    pub topic: Arc<ArcTopic>,
    pub use_default_value: bool,
    pub encrypt_value: bool,
    pub decrypt_value: bool,
    pub aid_hierarchy: bool,
    pub flatten: bool,
}

impl TopicSchemaFactorValuePrepper {
    pub fn with(
        topic: Arc<ArcTopic>,
        use_default_value: bool,
        encrypt_value: bool,
        decrypt_value: bool,
        aid_hierarchy: bool,
        flatten: bool,
    ) -> Self {
        Self {
            topic,
            use_default_value,
            encrypt_value,
            decrypt_value,
            aid_hierarchy,
            flatten,
        }
    }
}

/// prepare
impl TopicSchemaFactorValuePrepper {
    pub fn prepare(
        &self,
        factors: &Arc<TopicSchemaFactors>,
        data: &mut HashMap<String, TopicDataValue>,
    ) -> VoidR {
        // prepare including default value, date/time parse and encrypt/decrypt
        self.prepare_values(factors, data)?;
        self.aid_hierarchy(data)?;
        self.flatten(factors, data)?;

        Ok(())
    }
}

/// for default value, date/time cast, encrypt/decrypt
impl TopicSchemaFactorValuePrepper {
    /// - Ok(Some()) -> encrypted,
    /// - Ok(None) -> encryption is not needed,
    fn do_encrypt(
        &self,
        factor: &SimpleTopicSchemaFactor,
        value: &TopicDataValue,
    ) -> TriedTopicDataValue {
        if self.encrypt_value {
            factor.encrypt(value, &self.topic.tenant_id)
        } else {
            Ok(None)
        }
    }

    /// - Ok(Some()) -> encrypted or decrypted,
    /// - Ok(None) -> encryption and/or decryption is not needed,
    fn do_encrypt_or_decrypt(
        &self,
        factor: &SimpleTopicSchemaFactor,
        value: &TopicDataValue,
    ) -> TriedTopicDataValue {
        if self.encrypt_value {
            factor.encrypt(value, &self.topic.tenant_id)
        } else if self.decrypt_value {
            factor.decrypt(value, &self.topic.tenant_id)
        } else {
            Ok(None)
        }
    }

    fn prepare_none_value(&self, factor: &SimpleTopicSchemaFactor) -> TriedTopicDataValue {
        if !self.use_default_value || !factor.has_default_value() {
            // no need to init with default value, or no default value
            return Ok(None);
        }
        let default_value = factor.get_default_value();
        self.do_encrypt_or_decrypt(factor, &default_value)
            .treated_or_default(default_value)
    }

    fn prepare_str_value(
        &self,
        factor: &SimpleTopicSchemaFactor,
        value: &TopicDataValue, // must be str
    ) -> TriedTopicDataValue {
        if factor.is_date_or_time {
            let TopicDataValue::Str(str) = value else {
                return Ok(None);
            };
            let value = match factor.factor.r#type.deref() {
				FactorType::Date | FactorType::DateOfBirth => TopicDataValue::Date(str.to_date_loose()?),
				FactorType::Datetime => TopicDataValue::DateTime(str.to_datetime_loose()?),
				FactorType::FullDatetime => TopicDataValue::DateTime(str.to_datetime_loose()?),
				FactorType::Time => TopicDataValue::Time(str.to_time()?),
				_ => return RuntimeModelKernelErrorCode::TopicDataComplete.msg(
					format!(
						"Value[{}] of factor[factor_id={}, factor_name={}] cannot be cast to date/time because factor type is not.",
						value, factor.factor.factor_id, factor.factor.name,
					)
				)
			};
            // value just convert to date/time, no need to decrypt
            self.do_encrypt(factor, &value).treated_or_default(value)
        } else {
            self.do_encrypt_or_decrypt(factor, &value)
        }
    }

    fn encrypt_not_supported(
        &self,
        r#type: &str,
        factor: &SimpleTopicSchemaFactor,
        value: impl Display,
    ) -> VoidR {
        // return empty string if there is no encrypt method
        let encrypt_method = factor
            .factor
            .encrypt
            .as_ref()
            .map(|m| m.to_string())
            .unwrap_or("".to_string());
        RuntimeModelKernelErrorCode::TopicDataComplete.msg(
			format!(
				"{} value[{}] of factor[factor_id={}, factor_name={}] cannot be encrypted by method[{}].",
				r#type, value, factor.factor.factor_id, factor.factor.name, encrypt_method,
			)
		)
    }

    /// already has value.
    /// check there is encryptable or not, raise error if true
    fn prepare_bool_value(&self, factor: &SimpleTopicSchemaFactor, value: &bool) -> VoidR {
        if (self.encrypt_value || self.decrypt_value) && factor.is_encryptable {
            self.encrypt_not_supported("Boolean", factor, value)
        } else {
            Ok(())
        }
    }

    /// already has value.
    /// check there is encryptable or not, raise error if true
    fn prepare_num_value(&self, factor: &SimpleTopicSchemaFactor, value: &BigDecimal) -> VoidR {
        if (self.encrypt_value || self.decrypt_value) && factor.is_encryptable {
            self.encrypt_not_supported("Number", factor, value)
        } else {
            Ok(())
        }
    }

    fn prepare_map_value(
        &self,
        factor: &SimpleTopicSchemaFactor,
        value: &HashMap<String, TopicDataValue>,
    ) -> VoidR {
        if (self.encrypt_value || self.decrypt_value) && factor.is_encryptable {
            self.encrypt_not_supported("Map", factor, TopicDataValue::map_to_display(value))
        } else {
            Ok(())
        }
    }

    fn prepare_vec_value(
        &self,
        factor: &SimpleTopicSchemaFactor,
        value: &Vec<TopicDataValue>,
    ) -> VoidR {
        if (self.encrypt_value || self.decrypt_value) && factor.is_encryptable {
            self.encrypt_not_supported("Vec", factor, TopicDataValue::vec_to_display(value))
        } else {
            Ok(())
        }
    }

    fn prepare_date_value(
        &self,
        factor: &SimpleTopicSchemaFactor,
        value: &TopicDataValue,
    ) -> StdR<Option<TopicDataValue>> {
        self.do_encrypt_or_decrypt(factor, value)
    }

    fn prepare_datetime_value(
        &self,
        factor: &SimpleTopicSchemaFactor,
        value: &TopicDataValue,
    ) -> StdR<Option<TopicDataValue>> {
        self.do_encrypt_or_decrypt(factor, value)
    }

    fn prepare_time_value(&self, factor: &SimpleTopicSchemaFactor, value: &NaiveTime) -> VoidR {
        if (self.encrypt_value || self.decrypt_value) && factor.is_encryptable {
            self.encrypt_not_supported("Time", factor, value)
        } else {
            Ok(())
        }
    }

    fn prepare_simple_factor(
        &self,
        factor: &SimpleTopicSchemaFactor,
        data: &mut HashMap<String, TopicDataValue>,
    ) -> VoidR {
        let value = data.get(&factor.name);
        match value {
            Some(value) => match value {
                TopicDataValue::Str(_) => {
                    self.prepare_str_value(factor, value).if_treated(|value| {
                        data.insert(factor.name.clone(), value);
                    })
                }
                TopicDataValue::Num(n) => self.prepare_num_value(factor, n),
                TopicDataValue::Bool(b) => self.prepare_bool_value(factor, b),
                TopicDataValue::Map(map) => self.prepare_map_value(factor, map),
                TopicDataValue::Vec(vec) => self.prepare_vec_value(factor, vec),
                TopicDataValue::Date(_) => {
                    self.prepare_date_value(factor, value).if_treated(|value| {
                        data.insert(factor.name.clone(), value);
                    })
                }
                TopicDataValue::DateTime(_) => self
                    .prepare_datetime_value(factor, value)
                    .if_treated(|value| {
                        data.insert(factor.name.clone(), value);
                    }),
                TopicDataValue::Time(t) => self.prepare_time_value(factor, t),
                TopicDataValue::None => self.prepare_none_value(factor).if_treated(|value| {
                    data.insert(factor.name.clone(), value);
                }),
            },
            None => self.prepare_none_value(factor).if_treated(|value| {
                data.insert(factor.name.clone(), value);
            }),
        }
    }

    fn prepare_values_for_vec_or_map(
        &self,
        factor: Option<&Arc<ArcFactor>>,
        factor_name: &String, // in case factor is fake
        child_factors: &Vec<TopicSchemaFactor>,
        value: &mut TopicDataValue,
    ) -> VoidR {
        match value {
			TopicDataValue::Vec(vec) => {
				for element in vec {
					for child_factor in child_factors {
						match element {
							TopicDataValue::Map(map) => {
								self.prepare_factor(child_factor, map)?;
							}
							// no value presents, do nothing
							TopicDataValue::None => {}
							other => return RuntimeModelKernelErrorCode::TopicDataComplete.msg(if let Some(factor) = factor {
								format!(
									"Value[{}] in vec factor[factor_id={}, factor_name={}] is invalid, it must be a map.",
									other, factor.factor_id, factor.name
								)
							} else {
								format!(
									"Value[{}] in vec factor[{}] is invalid, it must be a map.",
									other, factor_name
								)
							})
						}
					}
				}
			}
			TopicDataValue::Map(map) => {
				for child_factor in child_factors {
					self.prepare_factor(child_factor, map)?;
				}
			}
			// no value presents, do nothing
			TopicDataValue::None => {}
			other => {
				return RuntimeModelKernelErrorCode::TopicDataComplete.msg(if let Some(factor) = factor {
					format!(
						"Value[{}] of factor[factor_id={}, factor_name={}] is invalid, it must be a vec or a map.",
						other, factor.factor_id, factor.name
					)
				} else {
					format!(
						"Value[{}] of factor[{}] is invalid, it must be a vec or a map.",
						other, factor_name
					)
				})
			}
		}
        Ok(())
    }

    fn prepare_vec_or_map_factor(
        &self,
        factor: &VecOrMapTopicSchemaFactor,
        data: &mut HashMap<String, TopicDataValue>,
    ) -> VoidR {
        if let Some(value) = data.get_mut(&factor.name) {
            self.prepare_values_for_vec_or_map(
                Some(&factor.factor),
                &factor.factor.name,
                &factor.children,
                value,
            )
        } else {
            // no value presents, do nothing
            Ok(())
        }
    }

    fn prepare_fake_factor(
        &self,
        factor: &FakeTopicSchemaFactor,
        data: &mut HashMap<String, TopicDataValue>,
    ) -> VoidR {
        if let Some(value) = data.get_mut(&factor.name) {
            self.prepare_values_for_vec_or_map(None, &factor.full_name, &factor.children, value)
        } else {
            // no value presents, do nothing
            Ok(())
        }
    }

    fn prepare_factor(
        &self,
        factor: &TopicSchemaFactor,
        data: &mut HashMap<String, TopicDataValue>,
    ) -> VoidR {
        match factor {
            TopicSchemaFactor::Simple(factor) => self.prepare_simple_factor(factor, data),
            TopicSchemaFactor::VecOrMap(factor) => self.prepare_vec_or_map_factor(factor, data),
            TopicSchemaFactor::Fake(factor) => self.prepare_fake_factor(factor, data),
        }
    }

    fn prepare_values(&self, factors: &Arc<TopicSchemaFactors>, data: &mut TopicData) -> VoidR {
        for factor in &factors.factors {
            self.prepare_factor(factor, data)?;
        }

        Ok(())
    }
}

/// for aid hierarchy
impl TopicSchemaFactorValuePrepper {
    fn aid_hierarchy(&self, data: &mut TopicData) -> VoidR {
        if self.aid_hierarchy {
            HierarchyAid::new().aid(data)?
        }
        Ok(())
    }
}

/// for flatten
impl TopicSchemaFactorValuePrepper {
    /// get all flatten factors
    fn get_flatten_factors<'a>(
        &self,
        factors: &'a Vec<TopicSchemaFactor>,
        vec: &mut Vec<&'a SimpleTopicSchemaFactor>,
    ) {
        for factor in factors {
            match factor {
                TopicSchemaFactor::Simple(factor) => {
                    if factor.is_flatten {
                        vec.push(factor);
                    }
                }
                TopicSchemaFactor::VecOrMap(vec_or_map) => {
                    self.get_flatten_factors(&vec_or_map.children, vec);
                }
                TopicSchemaFactor::Fake(fake) => {
                    self.get_flatten_factors(&fake.children, vec);
                }
            }
        }
    }

    /// all the data on the chain must be objects (maps) to support flattening.
    fn flatten_values<'a>(
        &self,
        name: &String,      // name of this part, use it to get value from map
        full_name: &String, // full name from root
        child_factors: &'a Vec<TopicSchemaFactor>, // next level factors
        map: &HashMap<String, TopicDataValue>,
        flatten_factors_and_values: &mut Vec<(&'a SimpleTopicSchemaFactor, TopicDataValue)>,
    ) -> VoidR {
        let value = map.get(name);
        match value {
			None | Some(TopicDataValue::None) => {
				// value is none, then all flatten factors' value are none
				let mut flatten_factors = vec![];
				self.get_flatten_factors(child_factors, &mut flatten_factors);
				if !flatten_factors.is_empty() {
					for flatten_factor in flatten_factors {
						flatten_factors_and_values.push((flatten_factor, TopicDataValue::None));
					}
				}
			}
			Some(TopicDataValue::Map(map)) => {
				// value is map, continue get flatten factors and values
				for child_factor in child_factors {
					match child_factor {
						TopicSchemaFactor::Simple(factor) => if factor.is_flatten {
							if let Some(value) = map.get(&factor.name) {
								flatten_factors_and_values.push((factor, value.clone()))
							} else {
								flatten_factors_and_values.push((factor, TopicDataValue::None))
							}
						},
						TopicSchemaFactor::VecOrMap(vec_or_map) => self.flatten_values(
							&vec_or_map.name,
							&vec_or_map.factor.name,
							&vec_or_map.children,
							map,
							flatten_factors_and_values,
						)?,
						TopicSchemaFactor::Fake(fake) => self.flatten_values(
							&fake.name,
							&fake.full_name,
							&fake.children,
							map,
							flatten_factors_and_values,
						)?,
					}
				}
			}
			Some(other) => return RuntimeModelKernelErrorCode::TopicDataComplete.msg(
				format!(
					"Value[{}] of factor[factor_name={}] cannot be of the none or any type other than map.",
					other, full_name,
				)
			)
		}

        Ok(())
    }

    /// collect flatten factors and values
    /// will not get value from map directly in this function.
    fn flatten_top_level_factor<'a>(
        &self,
        factor: &'a TopicSchemaFactor,
        map: &HashMap<String, TopicDataValue>,
        flatten_factors_and_values: &mut Vec<(&'a SimpleTopicSchemaFactor, TopicDataValue)>,
    ) -> VoidR {
        match factor {
            // simple factor is on top level, no need to flatten
            TopicSchemaFactor::Simple(_) => Ok(()),
            TopicSchemaFactor::VecOrMap(vec_or_map) => self.flatten_values(
                &vec_or_map.name,
                &vec_or_map.factor.name,
                &vec_or_map.children,
                map,
                flatten_factors_and_values,
            ),
            TopicSchemaFactor::Fake(fake) => self.flatten_values(
                &fake.name,
                &fake.full_name,
                &fake.children,
                map,
                flatten_factors_and_values,
            ),
        }
    }

    fn flatten(&self, factors: &Arc<TopicSchemaFactors>, data: &mut TopicData) -> VoidR {
        let mut flatten_factors_and_values = vec![];

        if self.flatten {
            // flatten must be the last step
            for factor in &factors.factors {
                self.flatten_top_level_factor(factor, data, &mut flatten_factors_and_values)?;
            }
        }
        for (flatten_factor, value) in flatten_factors_and_values {
            data.insert(flatten_factor.factor.name.deref().clone(), value);
        }

        Ok(())
    }
}
