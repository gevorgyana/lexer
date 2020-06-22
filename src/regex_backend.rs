use super::*;

use regex;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub enum RegexLexemeErr {
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

    fn recognize_raw_match(input : &str) -> Result<regex::Match, RegexLexemeErr> {
        match regex::Regex::new(Self::expression()) {
            Err(e) => {
                Err(RegexLexemeErr::InvalidExpression)
            },
            Ok(matcher) => {
                match matcher.find(input) {
                    Some(position) => {
                        if (position.start() > 0) {
                            Err(RegexLexemeErr::DistantMatch)
                        } else {
                            Ok(position)
                        }
                    },
                    None => {
                        Err(RegexLexemeErr::NoMatch)
                    }
                }
            }
        }
    }
}

impl<T> lexeme::Lexeme for T
where T : RegexLexeme
{
    fn recognize(input : &str) -> Result<token::Token, lexeme::LexemeErr> {
        if <Self as RegexLexeme>::needs_filtering() {
            let reserved_id = <Self as RegexLexeme>
                ::except_for();

            if let Ok(position) = <Self as regex_backend::RegexLexeme>
                ::recognize_raw_match(input) {

                if reserved_id.contains(&position.as_str()) {
                    Err(lexeme::LexemeErr::RegexErr(
                        RegexLexemeErr::LexemeNotAllowed
                    ))
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
                Err(lexeme::LexemeErr::RegexErr(RegexLexemeErr::NoMatch))
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
                    Err(lexeme::LexemeErr::RegexErr(reason))
                }
            }
        }
    }
}
