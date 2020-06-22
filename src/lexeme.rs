use super::token;
use super::regex_backend;

/// all recognizers implement this
pub trait Lexeme {

    fn recognize(input : &str) -> Result<token::Token, LexemeErr>;
}

#[derive(Debug, PartialEq)]
pub enum LexemeErr {
    RegexErr(regex_backend::RegexLexemeErr),
    AutomataErr(&'static str),
}
