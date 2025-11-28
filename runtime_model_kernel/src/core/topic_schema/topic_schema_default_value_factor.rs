use crate::{
    ArcFactor, TopicSchemaFactor, TopicSchemaFactorGroup, TopicSchemaFactorGroupInner,
    TopicSchemaFactorGroups, TopicSchemaFactorInner, TopicSchemaGroupFactor,
};
use std::ops::Deref;
use std::sync::Arc;
use watchmen_model::{BooleanUtils, FactorTypeCategory, NumericUtils, TopicData, TopicDataValue};

#[derive(Debug)]
pub struct TopicSchemaDefaultValueFactor {
    inner: TopicSchemaFactorInner,
    default_value: Option<Arc<TopicDataValue>>,
}

impl TopicSchemaDefaultValueFactor {
    pub fn new(inner: TopicSchemaFactorInner) -> Self {
        let mut factor = Self {
            inner,
            default_value: None,
        };
        factor.compute_default_value();
        factor
    }

    fn compute_default_value(&mut self) {
        let factor = &self.factor();
        let defined_default_value = &factor.default_value;
        if defined_default_value.is_none() {
            // no default value defined
            self.default_value = None;
            return;
        }

        let defined_default_value = defined_default_value.as_ref().unwrap();

        let computed_default_value = match factor.r#type.category() {
            FactorTypeCategory::Text
            | FactorTypeCategory::TextLike
            | FactorTypeCategory::EnumText => {
                TopicDataValue::Str(defined_default_value.deref().clone())
            }
            // date time related types
            FactorTypeCategory::FullDatetime => todo!("handle FullDatetime default value"),
            FactorTypeCategory::Datetime => todo!("handle Datetime default value"),
            FactorTypeCategory::Date => todo!("handle Date default value"),
            FactorTypeCategory::Time => todo!("handle Time default value"),
            // date time related types, no check, take as number
            FactorTypeCategory::DatetimeNumeric | FactorTypeCategory::Numeric => {
                if let Ok(v) = defined_default_value.deref().to_decimal() {
                    TopicDataValue::Num(v)
                } else {
                    // TODO output some warning info: string cannot be casted to decimal
                    TopicDataValue::None
                }
            }
            FactorTypeCategory::Boolean => {
                TopicDataValue::Bool(defined_default_value.deref().to_bool())
            }
            FactorTypeCategory::Complex => TopicDataValue::None,
        };
        self.default_value = Some(Arc::new(computed_default_value));
    }

    pub fn default_value(&self) -> &Option<Arc<TopicDataValue>> {
        &self.default_value
    }
}

impl TopicSchemaFactor for TopicSchemaDefaultValueFactor {
    fn get_inner(&self) -> &TopicSchemaFactorInner {
        &self.inner
    }
}

impl TopicSchemaGroupFactor<TopicSchemaDefaultValueFactor> for TopicSchemaDefaultValueFactor {
    fn replace_names(&self, names: Arc<Vec<String>>) -> TopicSchemaDefaultValueFactor {
        TopicSchemaDefaultValueFactor {
            inner: self.get_inner().replace_names(names),
            default_value: self.default_value.clone(),
        }
    }
}

pub type TopicSchemaDefaultValueFactorGroupInner =
    TopicSchemaFactorGroupInner<TopicSchemaDefaultValueFactor, TopicSchemaDefaultValueFactorGroup>;

#[derive(Debug)]
pub struct TopicSchemaDefaultValueFactorGroup {
    inner: TopicSchemaDefaultValueFactorGroupInner,
}

impl TopicSchemaDefaultValueFactorGroup {
    pub fn new(inner: TopicSchemaDefaultValueFactorGroupInner) -> Self {
        Self { inner }
    }

    pub fn init_default_value(&self, _data: &mut TopicData) {
        todo!("implement init_default_value for TopicSchemaDefaultValueFactorGroup")
    }
}

impl TopicSchemaFactorGroup<'_, TopicSchemaDefaultValueFactor, TopicSchemaDefaultValueFactorGroup>
    for TopicSchemaDefaultValueFactorGroup
{
    type Inner = TopicSchemaDefaultValueFactorGroupInner;

    fn new(name: Arc<String>, factors: Arc<Vec<Arc<TopicSchemaDefaultValueFactor>>>) -> Self {
        Self::new(TopicSchemaFactorGroupInner::new(name, factors))
    }

    fn get_inner(&self) -> &TopicSchemaDefaultValueFactorGroupInner {
        &self.inner
    }
}

pub struct TopicSchemaDefaultValueFactorGroups;

impl TopicSchemaFactorGroups<TopicSchemaDefaultValueFactor, TopicSchemaDefaultValueFactorGroup>
    for TopicSchemaDefaultValueFactorGroups
{
    fn accept_factor(factor: &Arc<ArcFactor>) -> bool {
        factor.has_default_value()
    }

    fn create_schema_factor(factor: &Arc<ArcFactor>) -> TopicSchemaDefaultValueFactor {
        TopicSchemaDefaultValueFactor::new(TopicSchemaFactorInner::new(factor.clone()))
    }

    fn create_schema_group(
        name: String,
        factors: Arc<Vec<Arc<TopicSchemaDefaultValueFactor>>>,
    ) -> TopicSchemaDefaultValueFactorGroup {
        TopicSchemaDefaultValueFactorGroup::new(TopicSchemaDefaultValueFactorGroupInner::new(
            Arc::new(name),
            factors,
        ))
    }
}
