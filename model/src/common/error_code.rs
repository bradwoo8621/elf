use elf_base::ErrorCode;

pub enum ModelErrorCode {
    StrEnumParse,
    VariableFunctionParse,
}

impl ErrorCode for ModelErrorCode {
    fn code(&self) -> &'static str {
        match self {
            Self::StrEnumParse => "MDLE-00001",
            Self::VariableFunctionParse => "MDLE-00002",
        }
    }
}
