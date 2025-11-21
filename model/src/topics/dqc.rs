use crate::topics::{ask_dqc_raw_rule_result_topic, ask_dqc_rule_daily_topic};
use crate::Topic;

pub fn ask_dqc_topics() -> Vec<Topic> {
    let mut topics = Vec::new();
    topics.push(ask_dqc_raw_rule_result_topic());
    topics.push(ask_dqc_rule_daily_topic());
    topics
}
