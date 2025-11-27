use crate::{
    CaseThenParameter, CaseThenParameterRoute, ConstantParameter, DayOfMonthParameter,
    EqualsExpression, FactorId, InsertOrMergeRowAction, MappingFactor, MonthOfParameter, Parameter,
    ParameterJoint, ParameterKind, Pipeline, PipelineStage, PipelineTriggerType, PipelineUnit,
    StdErr, StdR, Topic, TopicFactorParameter, TopicId, YearOfParameter,
};

fn find_topic<'a>(topics: &'a Vec<Topic>, topic_name: &'static str) -> StdR<&'a Topic> {
    let found = topics.into_iter().find(|topic| {
        if let Some(name) = &topic.name {
            name == topic_name
        } else {
            false
        }
    });
    if let Some(topic) = found {
        Ok(topic)
    } else {
        StdErr::unknown(format!("Topic[{}] not found.", topic_name))
    }
}

fn find_topic_id(topic: &Topic) -> StdR<&TopicId> {
    if let Some(topic_id) = &topic.topic_id {
        Ok(topic_id)
    } else {
        StdErr::unknown(format!("Topic[{:?}] has no factor_id value.", &topic.name))
    }
}

fn find_factor_id<'a>(topic: &'a Topic, factor_name: &'static str) -> StdR<&'a FactorId> {
    let found = if let Some(factors) = &topic.factors {
        factors.into_iter().find(|factor| {
            if let Some(name) = &factor.name {
                name == factor_name
            } else {
                false
            }
        })
    } else {
        None
    };

    if let Some(factor) = found {
        if let Some(factor_id) = &factor.factor_id {
            Ok(factor_id)
        } else {
            StdErr::unknown(format!("Factor[{}] has no factor_id value.", factor_name))
        }
    } else {
        StdErr::unknown(format!("Factor[{}] not found.", factor_name))
    }
}

fn topic_factor(topic_id: &TopicId, factor_id: &FactorId) -> Parameter {
    TopicFactorParameter {
        kind: Some(ParameterKind::Topic),
        topic_id: Some(topic_id.clone()),
        factor_id: Some(factor_id.clone()),
    }
    .to_parameter()
}

fn year_of(topic_id: &TopicId, factor_id: &FactorId) -> Parameter {
    YearOfParameter::init()
        .parameter(Box::new(topic_factor(topic_id, factor_id)))
        .to_parameter()
}

fn month_of(topic_id: &TopicId, factor_id: &FactorId) -> Parameter {
    MonthOfParameter::init()
        .parameter(Box::new(topic_factor(topic_id, factor_id)))
        .to_parameter()
}

fn day_of_month(topic_id: &TopicId, factor_id: &FactorId) -> Parameter {
    DayOfMonthParameter::init()
        .parameter(Box::new(topic_factor(topic_id, factor_id)))
        .to_parameter()
}

fn create_dqc_pipeline(topics: &Vec<Topic>) -> StdR<Pipeline> {
    let topic_raw = find_topic(topics, "dqc_raw_rule_result")?;
    let src_tid = find_topic_id(topic_raw)?;
    let src_fid_rule_code = find_factor_id(topic_raw, "ruleCode")?;
    let src_fid_topic_id = find_factor_id(topic_raw, "topicId")?;
    let src_fid_factor_id = find_factor_id(topic_raw, "factorId")?;
    let src_fid_process_date = find_factor_id(topic_raw, "processDate")?;
    let src_fid_detected = find_factor_id(topic_raw, "detected")?;

    let topic_daily = find_topic(topics, "dqc_rule_daily")?;
    let tgt_tid = find_topic_id(topic_daily)?;
    let tgt_fid_rule_code = find_factor_id(topic_daily, "ruleCode")?;
    let tgt_fid_topic_id = find_factor_id(topic_daily, "topicId")?;
    let tgt_fid_factor_id = find_factor_id(topic_daily, "factorId")?;
    let tgt_fid_year = find_factor_id(topic_daily, "year")?;
    let tgt_fid_month = find_factor_id(topic_daily, "month")?;
    let tgt_fid_day = find_factor_id(topic_daily, "day")?;
    let tgt_fid_process_date = find_factor_id(topic_daily, "processDate")?;
    let tgt_fid_count = find_factor_id(topic_daily, "count")?;

    let mapping = vec![
        MappingFactor::direct()
            .source(topic_factor(src_tid, src_fid_rule_code))
            .factor_id(tgt_fid_rule_code.clone()),
        MappingFactor::direct()
            .source(topic_factor(src_tid, src_fid_topic_id))
            .factor_id(tgt_fid_topic_id.clone()),
        MappingFactor::direct()
            .source(topic_factor(src_tid, src_fid_factor_id))
            .factor_id(tgt_fid_factor_id.clone()),
        MappingFactor::direct()
            .source(year_of(src_tid, src_fid_process_date))
            .factor_id(tgt_fid_year.clone()),
        MappingFactor::direct()
            .source(month_of(src_tid, src_fid_process_date))
            .factor_id(tgt_fid_month.clone()),
        MappingFactor::direct()
            .source(day_of_month(src_tid, src_fid_process_date))
            .factor_id(tgt_fid_day.clone()),
        MappingFactor::direct()
            .source(topic_factor(src_tid, src_fid_process_date))
            .factor_id(tgt_fid_process_date.clone()),
        MappingFactor::direct()
            .source(
                CaseThenParameter::init()
                    .parameters(vec![
                        CaseThenParameterRoute::case(ParameterJoint::and(vec![
                            EqualsExpression::init()
                                .left(topic_factor(src_tid, src_fid_detected))
                                .right(ConstantParameter::of(String::from("true")).to_parameter())
                                .to_condition(),
                        ]))
                        .parameter(ConstantParameter::of(String::from("1")).to_parameter()),
                        CaseThenParameterRoute::default()
                            .parameter(ConstantParameter::of(String::from("0")).to_parameter()),
                    ])
                    .to_parameter(),
            )
            .factor_id(tgt_fid_count.clone()),
    ];

    let by = ParameterJoint::and(vec![
        EqualsExpression::init()
            .left(topic_factor(src_tid, src_fid_rule_code))
            .right(topic_factor(tgt_tid, tgt_fid_rule_code))
            .to_condition(),
        EqualsExpression::init()
            .left(topic_factor(src_tid, src_fid_topic_id))
            .right(topic_factor(tgt_tid, tgt_fid_topic_id))
            .to_condition(),
        EqualsExpression::init()
            .left(topic_factor(src_tid, src_fid_factor_id))
            .right(topic_factor(tgt_tid, tgt_fid_factor_id))
            .to_condition(),
        EqualsExpression::init()
            .left(year_of(src_tid, src_fid_process_date))
            .right(topic_factor(tgt_tid, tgt_fid_year))
            .to_condition(),
        EqualsExpression::init()
            .left(month_of(src_tid, src_fid_process_date))
            .right(topic_factor(tgt_tid, tgt_fid_month))
            .to_condition(),
        EqualsExpression::init()
            .left(day_of_month(src_tid, src_fid_process_date))
            .right(topic_factor(tgt_tid, tgt_fid_day))
            .to_condition(),
    ]);

    let action = InsertOrMergeRowAction::init()
        .action_id(String::from("dqcp-a-1"))
        .topic_id(topic_daily.topic_id.as_ref().unwrap().clone())
        .mapping(mapping)
        .by(by)
        .to_action();

    let unit = PipelineUnit::new()
        .unit_id(String::from("dqcp-u-1"))
        .name(String::from("DQC Pipeline Unit 1"))
        .r#do(vec![action]);

    let stage = PipelineStage::new()
        .stage_id(String::from("dqcp-s-1"))
        .name(String::from("DQC Pipeline Stage 1"))
        .units(vec![unit]);

    Ok(Pipeline::new()
        .name(String::from("DQC Pipeline"))
        .topic_id(topic_raw.topic_id.as_ref().unwrap().clone())
        .r#type(PipelineTriggerType::InsertOrMerge)
        .stages(vec![stage]))
}

pub fn ask_dqc_pipelines(topics: &Vec<Topic>) -> StdR<Vec<Pipeline>> {
    // define all dqc pipelines

    let mut pipelines = Vec::<Pipeline>::new();

    pipelines.push(create_dqc_pipeline(topics)?);

    Ok(pipelines)
}
