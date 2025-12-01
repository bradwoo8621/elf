use crate::{
    ArcAddParameter, ArcCaseThenParameter, ArcDayOfMonthParameter, ArcDayOfWeekParameter,
    ArcDivideParameter, ArcHalfYearOfParameter, ArcModulusParameter, ArcMonthOfParameter,
    ArcMultiplyParameter, ArcNoneParameter, ArcQuarterOfParameter, ArcSubtractParameter,
    ArcWeekOfMonthParameter, ArcWeekOfYearParameter, ArcYearOfParameter,
};
use watchmen_model::{ComputedParameter, StdR};

#[derive(Debug)]
pub enum ArcComputedParameter {
    None(ArcNoneParameter),
    // math operations
    Add(ArcAddParameter),
    Subtract(ArcSubtractParameter),
    Multiply(ArcMultiplyParameter),
    Divide(ArcDivideParameter),
    Modulus(ArcModulusParameter),
    // date related operations
    YearOf(ArcYearOfParameter),
    HalfYearOf(ArcHalfYearOfParameter),
    QuarterOf(ArcQuarterOfParameter),
    MonthOf(ArcMonthOfParameter),
    WeekOfYear(ArcWeekOfYearParameter),
    WeekOfMonth(ArcWeekOfMonthParameter),
    DayOfMonth(ArcDayOfMonthParameter),
    DayOfWeek(ArcDayOfWeekParameter),
    // conditional operation
    CaseThen(ArcCaseThenParameter),
}

impl ArcComputedParameter {
    pub fn new(parameter: ComputedParameter) -> StdR<Self> {
        let arc_parameter = match parameter {
            ComputedParameter::None(p) => ArcComputedParameter::None(ArcNoneParameter::new(p)?),
            // math operations
            ComputedParameter::Add(p) => ArcComputedParameter::Add(ArcAddParameter::new(p)?),
            ComputedParameter::Subtract(p) => {
                ArcComputedParameter::Subtract(ArcSubtractParameter::new(p)?)
            }
            ComputedParameter::Multiply(p) => {
                ArcComputedParameter::Multiply(ArcMultiplyParameter::new(p)?)
            }
            ComputedParameter::Divide(p) => {
                ArcComputedParameter::Divide(ArcDivideParameter::new(p)?)
            }
            ComputedParameter::Modulus(p) => {
                ArcComputedParameter::Modulus(ArcModulusParameter::new(p)?)
            }
            // date related operations
            ComputedParameter::YearOf(p) => {
                ArcComputedParameter::YearOf(ArcYearOfParameter::new(p)?)
            }
            ComputedParameter::HalfYearOf(p) => {
                ArcComputedParameter::HalfYearOf(ArcHalfYearOfParameter::new(p)?)
            }
            ComputedParameter::QuarterOf(p) => {
                ArcComputedParameter::QuarterOf(ArcQuarterOfParameter::new(p)?)
            }
            ComputedParameter::MonthOf(p) => {
                ArcComputedParameter::MonthOf(ArcMonthOfParameter::new(p)?)
            }
            ComputedParameter::WeekOfYear(p) => {
                ArcComputedParameter::WeekOfYear(ArcWeekOfYearParameter::new(p)?)
            }
            ComputedParameter::WeekOfMonth(p) => {
                ArcComputedParameter::WeekOfMonth(ArcWeekOfMonthParameter::new(p)?)
            }
            ComputedParameter::DayOfMonth(p) => {
                ArcComputedParameter::DayOfMonth(ArcDayOfMonthParameter::new(p)?)
            }
            ComputedParameter::DayOfWeek(p) => {
                ArcComputedParameter::DayOfWeek(ArcDayOfWeekParameter::new(p)?)
            }
            // conditional operation
            ComputedParameter::CaseThen(p) => {
                ArcComputedParameter::CaseThen(ArcCaseThenParameter::new(p)?)
            }
        };

        Ok(arc_parameter)
    }
}
