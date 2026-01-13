use crate::ArcTopicDataValue;

impl ArcTopicDataValue {
    /// [None], [Empty Str] -> true,
    /// otherwise: false
    pub fn is_none_or_empty_str(&self) -> bool {
        match self {
            Self::None => true,
            Self::Str(v) => v.len() == 0,
            _ => false,
        }
    }
}
