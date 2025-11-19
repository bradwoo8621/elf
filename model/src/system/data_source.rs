use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, OptimisticLock, Storable, TenantBasedTuple, TenantId, Tuple, UserId,
};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum DataSourceParamValue {
    Str(String),
    Bool(bool),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DataSourceParam {
    pub name: Option<String>,
    pub value: Option<DataSourceParamValue>,
}

#[derive(Display, Serde)]
#[pattern = "lower-case"]
pub enum DataSourceType {
    MYSQL,
    ORACLE,
    MONGODB,
    MSSQL,
    POSTGRESQL,
    OSS,
    S3,
    #[display = "adls"]
    AzureDataLakeStorage,
}

pub type DataSourceId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct DataSource {
    pub data_source_id: Option<DataSourceId>,
    pub data_source_code: Option<String>,
    pub data_source_type: Option<DataSourceType>,
    pub host: Option<String>,
    pub port: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub params: Option<Vec<DataSourceParam>>,
}
