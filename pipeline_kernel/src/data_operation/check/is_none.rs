use crate::ArcTopicDataValue;

impl ArcTopicDataValue {
    /// check itself is [None] or not
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }
}
