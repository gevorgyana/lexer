use super::token;

/// all recognizers implement this
pub trait Lexeme {

    fn recognize(input : &str) -> Option<token::Token>;
}
