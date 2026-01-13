use crate::ArcTopicDataValue;
use elf_base::StdR;

impl ArcTopicDataValue {
    /// refer to [is_less_than]
    pub fn is_more_than_or_equals(&self, another: &ArcTopicDataValue) -> StdR<bool> {
        self.is_less_than(another).map(|b| !b)
    }
}
