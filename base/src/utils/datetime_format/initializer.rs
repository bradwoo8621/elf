use crate::{
    DateFormatter, DateTimeFormatter, DateTimeFormatterBase, EnvConfig, FullDateTimeFormatter,
    LooseDateFormatter, TimeFormatter, VoidR,
};

pub struct DateTimeFormatterInitializer;

impl DateTimeFormatterInitializer {
    /// initialize all date/time formatter by given environment
    /// TIP call it at system startup
    pub fn init(envs: &EnvConfig) -> VoidR {
        DateFormatter::init(envs)?;
        DateTimeFormatter::init(envs)?;
        FullDateTimeFormatter::init(envs)?;
        TimeFormatter::init(envs)?;
        LooseDateFormatter::init(envs)
    }
}
