use elf_base::{StdR, VoidR};
use elf_model::TopicDataValue;

pub type TriedTopicDataValue = StdR<Option<TopicDataValue>>;

pub trait TriedTDV {
    fn is_untreated(&self) -> bool;

    fn is_treated(&self) -> bool;

    fn is_err(&self) -> bool;

    fn treated_or_default(self, default_value: TopicDataValue) -> TriedTopicDataValue;

    fn if_treated<Replace>(self, f: Replace) -> VoidR
    where
        Replace: FnOnce(TopicDataValue);
}

impl TriedTDV for TriedTopicDataValue {
    fn is_untreated(&self) -> bool {
        self.is_ok() && self.as_ref().unwrap().is_none()
    }

    fn is_treated(&self) -> bool {
        self.is_ok() && !self.as_ref().unwrap().is_none()
    }

    fn is_err(&self) -> bool {
        self.is_err()
    }

    fn treated_or_default(self, default_value: TopicDataValue) -> TriedTopicDataValue {
        let value = self?;
        if let Some(value) = value {
            Ok(Some(value))
        } else {
            Ok(Some(default_value))
        }
    }

    fn if_treated<Replace>(self, f: Replace) -> VoidR
    where
        Replace: FnOnce(TopicDataValue),
    {
        match self {
            Ok(Some(value)) => {
                f(value);
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
