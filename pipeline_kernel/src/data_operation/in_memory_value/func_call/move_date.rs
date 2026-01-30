use crate::{ArcFrom, ArcTopicDataValue, InMemoryFuncCall, FUNC_PARAM_TRANSFORMED_TAG};
use elf_base::{
    DateMoveUtils, DateTimeMoveSupport, DateTimeMovement, DateTimeMovements, DateTimeUtils, StdR,
};
use std::ops::Deref;
use std::sync::Arc;

struct DateMoveHelper;

impl DateMoveHelper {
    // noinspection DuplicatedCode
    fn move_date(
        func_call: &InMemoryFuncCall,
        value: &ArcTopicDataValue,
        movement: &ArcTopicDataValue,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let moved = match value {
            ArcTopicDataValue::Date(date) => {
                if let Some(date) =
                    date.move_to(Self::get_movements_from_param(func_call, movement)?.deref())
                {
                    ArcTopicDataValue::arc_from(date)
                } else {
                    return func_call.func_not_supported(movement);
                }
            }
            ArcTopicDataValue::DateTime(datetime) => {
                if let Some(datetime) =
                    datetime.move_to(Self::get_movements_from_param(func_call, movement)?.deref())
                {
                    ArcTopicDataValue::arc_from(datetime)
                } else {
                    return func_call.func_not_supported(movement);
                }
            }
            ArcTopicDataValue::Time(time) => {
                if let Some(time) =
                    time.move_to(Self::get_movements_from_param(func_call, movement)?.deref())
                {
                    ArcTopicDataValue::arc_from(time)
                } else {
                    return func_call.func_not_supported(movement);
                }
            }
            ArcTopicDataValue::Str(str) => {
                if let Ok(datetime) = str.to_datetime_loose() {
                    if let Some(datetime) = datetime
                        .move_to(Self::get_movements_from_param(func_call, movement)?.deref())
                    {
                        ArcTopicDataValue::arc_from(datetime)
                    } else {
                        return func_call.func_not_supported(movement);
                    }
                } else if let Ok(time) = str.to_time() {
                    if let Some(time) =
                        time.move_to(Self::get_movements_from_param(func_call, movement)?.deref())
                    {
                        ArcTopicDataValue::arc_from(time)
                    } else {
                        return func_call.func_not_supported(movement);
                    }
                } else {
                    return func_call.func_not_supported(str);
                }
            }
            other => return func_call.func_not_supported(other),
        };
        Ok(moved)
    }

    fn create_movement(
        func_call: &InMemoryFuncCall,
        movement: &Arc<ArcTopicDataValue>,
        movements: &ArcTopicDataValue,
    ) -> StdR<DateTimeMovement> {
        match movement.deref() {
            ArcTopicDataValue::Vec(vec) => {
                if vec.len() != 3 {
                    return func_call.func_not_supported(movements);
                }

                match (vec[0].deref(), vec[1].deref(), vec[2].deref()) {
                    (
                        ArcTopicDataValue::Str(unit),
                        ArcTopicDataValue::Num(r#type),
                        ArcTopicDataValue::Num(value),
                    ) => DateTimeMovement::of(unit.deref(), r#type.deref(), value.deref()),
                    _ => func_call.func_not_supported(movements),
                }
            }
            _ => func_call.func_not_supported(movements),
        }
    }

    fn get_movements_from_param(
        func_call: &InMemoryFuncCall,
        movements: &ArcTopicDataValue,
    ) -> StdR<Arc<DateTimeMovements>> {
        match movements {
            ArcTopicDataValue::Str(str) => Ok(Arc::new(DateTimeMoveSupport::parse(str)?)),
            ArcTopicDataValue::Vec(vec) => {
                if vec.len() != 2 {
                    return func_call.func_not_supported(movements);
                }
                match (vec[0].deref(), vec[1].deref()) {
                    (ArcTopicDataValue::Str(tag), ArcTopicDataValue::Vec(movements_vec)) => {
                        if tag.deref() != FUNC_PARAM_TRANSFORMED_TAG {
                            return func_call.func_not_supported(movements);
                        }
                        let mut created_movements = vec![];
                        for movement in movements_vec.iter() {
                            created_movements
                                .push(Self::create_movement(func_call, movement, movements)?);
                        }
                        Ok(Arc::new(created_movements))
                    }
                    _ => func_call.func_not_supported(movements),
                }
            }
            other => func_call.func_not_supported(other),
        }
    }
}

impl InMemoryFuncCall<'_> {
    pub fn resolve_move_date(
        &self,
        context: Arc<ArcTopicDataValue>,
        params: Vec<Arc<ArcTopicDataValue>>,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        self.one_param(&params, |param| {
            DateMoveHelper::move_date(&self, context.deref(), param)
        })
    }
}
