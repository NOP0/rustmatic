#[derive(Debug, Clone, PartialEq)]
pub struct IntegerLiteral {
    pub base: usize,
    pub raw: String,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub raw_value: String,
}
