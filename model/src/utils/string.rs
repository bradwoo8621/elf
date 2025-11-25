pub trait StringUtils {
    fn is_blank(&self) -> bool;
    fn is_not_blank(&self) -> bool {
        !self.is_blank()
    }
}

impl StringUtils for Option<String> {
    fn is_blank(&self) -> bool {
        match self {
            Some(s) => s.trim().is_empty(),
            None => true,
        }
    }
}

impl StringUtils for String {
    fn is_blank(&self) -> bool {
        self.trim().is_empty()
    }
}