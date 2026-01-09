use crate::{ArcFactor, FactorCrypto, TopicSchemaFactor, TriedTopicDataValue};
use elf_base::{BooleanUtils, DateTimeUtils, NumericUtils, StdR};
use elf_model::{FactorEncryptMethod, FactorTypeCategory, TenantId, TopicDataValue};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;

pub struct SimpleTopicSchemaFactor {
    pub factor: Arc<ArcFactor>,
    pub name: String,
    pub is_date_or_time: bool,
    pub is_encryptable: bool,
    pub default_value: Option<Arc<TopicDataValue>>,
    pub is_flatten: bool,
}

impl Display for SimpleTopicSchemaFactor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SimpleTopicSchemaFactor[name={}, factor_name={}, factor_id={}, is_date_or_time={}, is_encryptable={}, default_value={}, is_flatten={}]",
            self.name,
            self.factor.name,
            self.factor.factor_id,
            self.is_date_or_time,
            self.is_encryptable,
            if let Some(value) = &self.default_value {
                format!("{}", value)
            } else {
                "".to_string()
            },
            self.is_flatten,
        )
    }
}

impl SimpleTopicSchemaFactor {
    pub fn new(factor: Arc<ArcFactor>, name: String) -> Self {
        Self {
            name,
            is_date_or_time: factor.is_date_or_time(),
            is_encryptable: if let Some(encrypt_method) = &factor.encrypt {
                match encrypt_method.deref() {
                    FactorEncryptMethod::None => false,
                    _ => true,
                }
            } else {
                false
            },
            default_value: Self::compute_default_value(&factor),
            is_flatten: factor.flatten,
            factor,
        }
    }

    /// compute default value
    /// - none when not defined,
    /// - none when empty string,
    /// - ignore default value when cast failed,
    /// - do not perform a validity check on the default value.
    fn compute_default_value(factor: &Arc<ArcFactor>) -> Option<Arc<TopicDataValue>> {
        let defined_default_value = &factor.default_value;
        if defined_default_value.is_none() {
            // no default value defined
            return None;
        }

        let defined_default_value = defined_default_value.as_ref().unwrap();
        if defined_default_value.is_empty() {
            // defined default value is empty string
            return None;
        }

        let computed_default_value = match factor.r#type.category() {
            FactorTypeCategory::Text
            | FactorTypeCategory::TextLike
            | FactorTypeCategory::EnumText => {
                TopicDataValue::Str(defined_default_value.deref().clone())
            }
            // date time related types
            FactorTypeCategory::FullDatetime => {
                // TIP note that a strict full datetime format is used for parsing here,
                //  and this is also the only place in the entire system where the strict full datetime format is used.
                if let Ok(v) = defined_default_value.deref().to_full_datetime() {
                    TopicDataValue::DateTime(v)
                } else {
                    TopicDataValue::None
                }
            }
            FactorTypeCategory::Datetime => {
                if let Ok(v) = defined_default_value.deref().to_datetime_loose() {
                    TopicDataValue::DateTime(v)
                } else {
                    TopicDataValue::None
                }
            }
            FactorTypeCategory::Date => {
                if let Ok(v) = defined_default_value.deref().to_date_loose() {
                    TopicDataValue::Date(v)
                } else {
                    TopicDataValue::None
                }
            }
            FactorTypeCategory::Time => {
                if let Ok(v) = defined_default_value.deref().to_time() {
                    TopicDataValue::Time(v)
                } else {
                    TopicDataValue::None
                }
            }
            // date time related types, no check, take as number
            FactorTypeCategory::DatetimeNumeric | FactorTypeCategory::Numeric => {
                if let Ok(v) = defined_default_value.deref().to_decimal() {
                    TopicDataValue::Num(v)
                } else {
                    TopicDataValue::None
                }
            }
            FactorTypeCategory::Boolean => {
                TopicDataValue::Bool(defined_default_value.deref().to_bool())
            }
            FactorTypeCategory::Complex => TopicDataValue::None,
        };

        match computed_default_value {
            TopicDataValue::None => None,
            _ => Some(Arc::new(computed_default_value)),
        }
    }

    pub fn if_functional(self) -> Option<TopicSchemaFactor> {
        if self.is_date_or_time
            || self.is_encryptable
            || self.is_flatten
            || self.default_value.is_some()
        {
            Some(TopicSchemaFactor::Simple(self))
        } else {
            None
        }
    }

    pub fn has_default_value(&self) -> bool {
        self.default_value.is_some()
    }

    pub fn get_default_value(&self) -> TopicDataValue {
        match self.default_value.as_ref() {
            Some(value) => value.deref().clone(),
            None => TopicDataValue::None,
        }
    }

    fn get_crypto(&self, tenant_id: &Arc<TenantId>) -> StdR<Option<FactorCrypto>> {
        if let Some(encrypt) = &self.factor.encrypt {
            FactorCrypto::get(encrypt.as_ref(), tenant_id)
        } else {
            Ok(None)
        }
    }

    /// - Ok(Some()) -> encrypted,
    /// - Ok(None) -> encryption is not needed,
    pub fn encrypt(
        &self,
        value: &TopicDataValue,
        tenant_id: &Arc<TenantId>,
    ) -> TriedTopicDataValue {
        if let Some(crypto) = self.get_crypto(tenant_id)? {
            if let Some(encrypted) = crypto.encrypt(value)? {
                Ok(Some(encrypted))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// - Ok(Some()) -> decrypted,
    /// - Ok(None) -> decryption is not needed,
    pub fn decrypt(
        &self,
        value: &TopicDataValue,
        tenant_id: &Arc<TenantId>,
    ) -> TriedTopicDataValue {
        if let Some(crypto) = self.get_crypto(tenant_id)? {
            if let Some(decrypted) = crypto.decrypt(value)? {
                Ok(Some(decrypted))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
