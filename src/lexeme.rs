use super::token;
use super::regex;
use super::dfa;

/// all recognizers implement this
pub trait Lexeme {

    fn recognize(input : &str) -> Result<token::Token, Error>;
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Regex(regex::Error),
    Automata(dfa::Error),
    FoundConflictingLexeme,
    NotRecognized,
    FoundDistant,
    NonAsciiFound,
}
