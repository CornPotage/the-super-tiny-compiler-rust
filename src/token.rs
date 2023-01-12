use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

#[derive(Debug, PartialEq, Clone, Copy, EnumString, Display, IntoStaticStr, EnumIter)]
pub enum TokenType {
    #[strum(serialize = "number")]
    NUMBER,
    #[strum(serialize = "string")]
    STRING,
    #[strum(serialize = "name")]
    NAME,
    #[strum(serialize = "paren")]
    PAREN,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) value: String,
}
