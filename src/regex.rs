use super::lexeme;
use super::token;

use regex as regex_backend;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub enum Error {
    NoMatch,
    DistantMatch,
    InvalidExpression,
    LexemeNotAllowed,
}

pub trait RegexLexeme {

    // settings

    /// activates the usage of except_for() filtering, false by default
    fn needs_filtering() -> bool { false }
    /// a regular expression that defines the matches that must be avoided
    fn except_for() -> &'static str { r"" }
    fn token_type() -> token::TokenType;
    /// a regular expression that defines the matches that should be reported
    fn expression() -> &'static str;

    fn recognize_raw_match(input : &str) -> Result<regex_backend::Match, Error> {
        match regex_backend::Regex::new(Self::expression()) {
            Err(e) => {
                Err(Error::InvalidExpression)
            },
            Ok(matcher) => {
                match matcher.find(input) {
                    Some(position) => {
                        if (position.start() > 0) {
                            Err(Error::DistantMatch)
                        } else {
                            Ok(position)
                        }
                    },
                    None => {
                        Err(Error::NoMatch)
                    }
                }
            }
        }
    }
}

impl<T> lexeme::Lexeme for T
where T : RegexLexeme
{
    fn recognize(input : &str) -> Result<token::Token, lexeme::Error> {
        if <Self as RegexLexeme>::needs_filtering() {
            let reserved_id = <Self as RegexLexeme>
                ::except_for();

            if let Ok(position) = <Self as RegexLexeme>
                ::recognize_raw_match(input) {

                if reserved_id.contains(&position.as_str()) {
                    Err(lexeme::Error::FoundConflictingLexeme)
                } else {
                    Ok(
                        token::Token {
                            span : vec![
                                (position.end() -
                                 position.start())
                                    .try_into()
                                    .unwrap()],
                            token_type : Self::token_type(),
                        }
                    )
                }
            } else {
                Err(lexeme::Error::Regex(Error::NoMatch))
            }
        } else {
            match <Self as RegexLexeme>::recognize_raw_match(input) {
                Ok(position) => {
                    Ok(
                        token::Token {
                            span : vec![
                                (position.end() -
                                 position.start())
                                    .try_into()
                                    .unwrap()],
                            token_type : Self::token_type(),
                        }
                    )
                },
                Err(reason) => {
                    Err(lexeme::Error::Regex(reason))
                }
            }
        }
    }
}
