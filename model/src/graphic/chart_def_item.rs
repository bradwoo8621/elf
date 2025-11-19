use crate::{BaseDataModel, ChartColor, Storable};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum ChartDefItemType {
    Section,
    Number,
    Percentage,
    Boolean,
    Text,
    Color,
    Dropdown,
}

/// TODO is it workable?
#[adapt_model(storable)]
pub enum ChartDefItem {
    Section(ChartSectionItem),
    Input(ChartInputItem),
}

#[adapt_model(storable)]
pub struct ChartSectionItem {
    pub r#type: Option<ChartDefItemType>,
    pub label: Option<String>,
}

/// TODO is it workable?
#[adapt_model(storable)]
pub enum ChartInputItem {
    Number(ChartNumberItem),
    Percentage(ChartPercentageItem),
    Boolean(ChartBooleanItem),
    Text(ChartTextItem),
    Color(ChartColorItem),
    Dropdown(ChartDropdownItem),
}

#[adapt_model(storable)]
pub struct ChartNumberItem {
    pub r#type: Option<ChartDefItemType>,
    pub label: Option<String>,
    pub key: Option<String>,
    pub placeholder: Option<String>,
    pub unit: Option<String>,
    pub default_value: Option<BigDecimal>,
}

#[adapt_model(storable)]
pub struct ChartPercentageItem {
    pub r#type: Option<ChartDefItemType>,
    pub label: Option<String>,
    pub key: Option<String>,
    pub placeholder: Option<String>,
    pub default_value: Option<BigDecimal>,
}

#[adapt_model(storable)]
pub struct ChartBooleanItem {
    pub r#type: Option<ChartDefItemType>,
    pub label: Option<String>,
    pub key: Option<String>,
    pub default_value: Option<bool>,
}

#[adapt_model(storable)]
pub struct ChartTextItem {
    pub r#type: Option<ChartDefItemType>,
    pub label: Option<String>,
    pub key: Option<String>,
    pub placeholder: Option<String>,
    pub default_value: Option<String>,
}

#[adapt_model(storable)]
pub struct ChartColorItem {
    pub r#type: Option<ChartDefItemType>,
    pub label: Option<String>,
    pub key: Option<String>,
    pub default_value: Option<ChartColor>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum ChartDropdownItemOptionValue {
    Str(String),
    Num(BigDecimal),
    Bool(bool),
}

#[adapt_model(storable)]
pub struct ChartDropdownItemOption {
    pub value: Option<ChartDropdownItemOptionValue>,
    pub label: Option<String>,
}

#[adapt_model(storable)]
pub struct ChartDropdownItem {
    pub r#type: Option<ChartDefItemType>,
    pub label: Option<String>,
    pub key: Option<String>,
    pub placeholder: Option<String>,
    pub options: Option<Vec<ChartDropdownItemOption>>,
    pub default_value: Option<ChartDropdownItemOptionValue>,
}
