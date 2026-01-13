use crate::ArcTopicDataValue;

impl ArcTopicDataValue {
    /// [None], [Empty Str], [Empty Map], [Empty Vec] -> true,
    /// otherwise: false
    pub fn is_empty(&self) -> bool {
        match self {
            Self::None => true,
            Self::Str(v) => v.is_empty(),
            Self::Map(v) => v.is_empty(),
            Self::Vec(v) => v.is_empty(),
            _ => false,
        }
    }

    /// [None], [Empty Str], [Empty Map], [Empty Vec] -> false,
    /// otherwise: true
    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}
