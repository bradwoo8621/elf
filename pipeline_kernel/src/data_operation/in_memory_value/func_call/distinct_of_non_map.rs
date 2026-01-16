use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall};
use elf_base::StdR;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;

fn check<V>(value: &V, map: &mut HashMap<V, usize>) -> bool
where
    V: Hash + Eq + Clone,
{
    if !map.contains_key(value) {
        map.insert(value.clone(), 1);
        true
    } else {
        false
    }
}

impl InMemoryFuncCall<'_> {
    /// [VariablePredefineFunctions::Distinct]
    ///
    /// for vec, distinct;
    /// for other non-map values, return vec with one element which is itself.
    /// - context can be anything but map,
    /// - no parameter
    /// - element of vec is vec or map, it is unique
    /// - distinct keeps none value, and will not do any type cast
    pub fn resolve_distinct_of_non_map(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        self.no_param(&params, || match context.deref() {
            ArcTopicDataValue::None
            | ArcTopicDataValue::Str(_)
            | ArcTopicDataValue::Num(_)
            | ArcTopicDataValue::Bool(_)
            | ArcTopicDataValue::DateTime(_)
            | ArcTopicDataValue::Date(_)
            | ArcTopicDataValue::Time(_) => Ok(ArcTopicDataValue::arc_from(vec![context.clone()])),
            ArcTopicDataValue::Vec(vec) => {
                if vec.is_empty() {
                    Ok(context.clone())
                } else {
                    let mut has_none = false;
                    let mut str_map = HashMap::new();
                    let mut num_map = HashMap::new();
                    let mut has_true = false;
                    let mut has_false = false;
                    let mut datetime_map = HashMap::new();
                    let mut date_map = HashMap::new();
                    let mut time_map = HashMap::new();
                    let mut distinct_vec = vec![];

                    for elm in vec.iter() {
                        let is_distinct = match elm.deref() {
                            ArcTopicDataValue::None => {
                                if has_none {
                                    false
                                } else {
                                    has_none = true;
                                    true
                                }
                            }
                            ArcTopicDataValue::Str(s) => check(s.deref(), &mut str_map),
                            ArcTopicDataValue::Num(n) => check(n, &mut num_map),
                            ArcTopicDataValue::Bool(b) => {
                                if *b {
                                    if has_true {
                                        false
                                    } else {
                                        has_true = true;
                                        true
                                    }
                                } else {
                                    if has_false {
                                        false
                                    } else {
                                        has_false = true;
                                        true
                                    }
                                }
                            }
                            ArcTopicDataValue::DateTime(dt) => check(dt, &mut datetime_map),
                            ArcTopicDataValue::Date(d) => check(d, &mut date_map),
                            ArcTopicDataValue::Time(t) => check(t, &mut time_map),
                            ArcTopicDataValue::Vec(_) | ArcTopicDataValue::Map(_) => true,
                        };
                        if is_distinct {
                            distinct_vec.push(elm.clone())
                        }
                    }
                    Ok(ArcTopicDataValue::arc_from(distinct_vec))
                }
            }
            ArcTopicDataValue::Map(_) => self.func_not_supported(context.deref())?,
        })
    }
}
