use crate::{ArcParameter, ArcParameterJoint, RuntimeModelKernelErrorCode};
use elf_base::{ErrorCode, StdR, StringUtils};
use elf_model::{FactorId, Parameter, ParameterJoint, TenantId, TopicId};
use std::sync::Arc;

pub trait ArcHelper {
    fn arc<V>(value: Option<V>) -> Option<Arc<V>> {
        value.map(Arc::new)
    }

    /// use the value or use default value [""] when there is no value given
    fn or_empty_str(value: Option<String>) -> Arc<String> {
        Arc::new(value.unwrap_or("".to_string()))
    }

    fn must<V, F>(value: Option<V>, on_none: F) -> StdR<Arc<V>>
    where
        F: FnOnce() -> StdR<Arc<V>>,
    {
        if value.is_none() {
            on_none()
        } else {
            Ok(Arc::new(value.unwrap()))
        }
    }

    fn action_source<F, P>(source: Option<Parameter>, pos: F) -> StdR<Arc<ArcParameter>>
    where
        F: FnOnce() -> P,
        P: Into<String>,
    {
        Self::must_then(source, ArcParameter::new, || {
            RuntimeModelKernelErrorCode::ActionSourceMissed
                .msg(format!("{} must have a source.", pos().into()))
        })
    }

    fn parameter_left(left: Option<Parameter>) -> StdR<Arc<ArcParameter>> {
        Self::must_then(left, ArcParameter::new, || {
            RuntimeModelKernelErrorCode::ParameterLeftMissed.msg("Parameter must have a left.")
        })
    }

    fn parameter_right(right: Option<Parameter>) -> StdR<Arc<ArcParameter>> {
        Self::must_then(right, ArcParameter::new, || {
            RuntimeModelKernelErrorCode::ParameterRightMissed.msg("Parameter must have a right.")
        })
    }

    fn action_by<F, P>(by: Option<ParameterJoint>, pos: F) -> StdR<Arc<ArcParameterJoint>>
    where
        F: FnOnce() -> P,
        P: Into<String>,
    {
        Self::must_then(by, ArcParameterJoint::new, || {
            RuntimeModelKernelErrorCode::ConditionMissed
                .msg(format!("{} must have a by.", pos().into()))
        })
    }

    fn must_then<V, ArcV, F1, F2>(value: Option<V>, on_some: F1, on_none: F2) -> StdR<Arc<ArcV>>
    where
        F1: FnOnce(V) -> StdR<Arc<ArcV>>,
        F2: FnOnce() -> StdR<Arc<ArcV>>,
    {
        if value.is_none() {
            on_none()
        } else {
            on_some(value.unwrap())
        }
    }

    /// check topic id is not missing and not blank
    fn topic_id<F, P>(topic_id: Option<TopicId>, pos: F) -> StdR<Arc<TopicId>>
    where
        F: Fn() -> P,
        P: Into<String>,
    {
        Self::not_blank(
            topic_id,
            || {
                RuntimeModelKernelErrorCode::TopicIdMissed
                    .msg(format!("{} must have a topic id.", pos().into()))
            },
            || {
                RuntimeModelKernelErrorCode::TopicIdIsBlank
                    .msg(format!("{}'s topic id cannot be blank.", pos().into()))
            },
        )
    }

    /// check factor id is not missing and not blank
    fn factor_id<F, P>(factor_id: Option<FactorId>, pos: F) -> StdR<Arc<FactorId>>
    where
        F: Fn() -> P,
        P: Into<String>,
    {
        Self::not_blank(
            factor_id,
            || {
                RuntimeModelKernelErrorCode::FactorIdMissed
                    .msg(format!("{} must have a factor id.", pos().into()))
            },
            || {
                RuntimeModelKernelErrorCode::FactorIdIsBlank
                    .msg(format!("{}'s factor id cannot be blank.", pos().into()))
            },
        )
    }

    /// check name is not missing and not blank
    fn name<F, P>(name: Option<String>, pos: F) -> StdR<Arc<String>>
    where
        F: Fn() -> P,
        P: Into<String>,
    {
        Self::not_blank(
            name,
            || {
                RuntimeModelKernelErrorCode::NameMissed
                    .msg(format!("{} must have a name.", pos().into()))
            },
            || {
                RuntimeModelKernelErrorCode::NameIsBlank
                    .msg(format!("{}'s name cannot be blank.", pos().into()))
            },
        )
    }

    /// check tenant id is not missing and not blank
    fn tenant_id<F, P>(tenant_id: Option<TenantId>, pos: F) -> StdR<Arc<TenantId>>
    where
        F: Fn() -> P,
        P: Into<String>,
    {
        Self::not_blank(
            tenant_id,
            || {
                RuntimeModelKernelErrorCode::TenantIdMissed
                    .msg(format!("{} must have a tenant id.", pos().into()))
            },
            || {
                RuntimeModelKernelErrorCode::TenantIdIsBlank
                    .msg(format!("{}'s tenant id cannot be blank.", pos().into()))
            },
        )
    }

    /// check variable name is not missing and not blank
    fn variable_name<F, P>(variable_name: Option<String>, pos: F) -> StdR<Arc<String>>
    where
        F: Fn() -> P,
        P: Into<String>,
    {
        Self::not_blank(
            variable_name,
            || {
                RuntimeModelKernelErrorCode::ActionVariableNameMissed
                    .msg(format!("{} must have a variable name.", pos().into()))
            },
            || {
                RuntimeModelKernelErrorCode::ActionVariableNameIsBlank
                    .msg(format!("{}'s variable name cannot be blank.", pos().into()))
            },
        )
    }

    /// check the given value:
    /// - empty: call [on_empty],
    /// - blank: call [on_blank],
    /// or return [Arc] if pass the check.
    fn not_blank<F1, F2>(value: Option<String>, on_empty: F1, on_blank: F2) -> StdR<Arc<String>>
    where
        F1: FnOnce() -> StdR<Arc<String>>,
        F2: FnOnce() -> StdR<Arc<String>>,
    {
        if let Some(v) = value {
            if v.is_not_blank() {
                Ok(Arc::new(v))
            } else {
                on_blank()
            }
        } else {
            on_empty()
        }
    }

    fn must_vec<V, ArcV, F1, F2>(
        values: Option<Vec<V>>,
        arc: F1,
        on_empty: F2,
    ) -> StdR<Arc<Vec<Arc<ArcV>>>>
    where
        F1: Fn(V) -> StdR<Arc<ArcV>>,
        F2: FnOnce() -> StdR<Arc<Vec<Arc<ArcV>>>>,
    {
        if values.is_none() {
            return on_empty();
        }
        let values = values.unwrap();
        if values.len() == 0 {
            return on_empty();
        }
        let mut arc_values = vec![];
        for value in values {
            arc_values.push(arc(value)?);
        }
        Ok(Arc::new(arc_values))
    }

    /// check the given args:
    /// - [conditional] use default value [false] if given arg not presents,
    /// - [on] cannot be empty if [conditional] is [true]
    fn conditional<F, P>(
        conditional: Option<bool>,
        on: Option<ParameterJoint>,
        msg: F,
    ) -> StdR<Option<Arc<ArcParameterJoint>>>
    where
        F: FnOnce() -> P,
        P: Into<String>,
    {
        let conditional = conditional.unwrap_or(false);
        if conditional {
            if on.is_none() {
                RuntimeModelKernelErrorCode::ConditionMissed.msg(msg())
            } else {
                Ok(Some(ArcParameterJoint::new(on.unwrap())?))
            }
        } else {
            Ok(None)
        }
    }
}
