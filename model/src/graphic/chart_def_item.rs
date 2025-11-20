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

// TODO can't find out where this is used, seems to define the chart properties
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ChartDefItem {
    #[serde(rename = "section")]
    Section(ChartSectionItem),
    #[serde(rename = "number")]
    Number(ChartNumberItem),
    #[serde(rename = "percentage")]
    Percentage(ChartPercentageItem),
    #[serde(rename = "boolean")]
    Boolean(ChartBooleanItem),
    #[serde(rename = "text")]
    Text(ChartTextItem),
    #[serde(rename = "color")]
    Color(ChartColorItem),
    #[serde(rename = "dropdown")]
    Dropdown(ChartDropdownItem),
}

#[adapt_model(storable)]
pub struct ChartSectionItem {
    pub r#type: Option<ChartDefItemType>,
    pub label: Option<String>,
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
