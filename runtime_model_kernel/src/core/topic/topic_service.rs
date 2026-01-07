use crate::{TopicDataProvider, TopicMetaProvider, TopicSchemaProvider};

pub struct TopicService;

impl TopicDataProvider for TopicService {}
impl TopicMetaProvider for TopicService {}
impl TopicSchemaProvider for TopicService {}
