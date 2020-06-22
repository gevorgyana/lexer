use regex;
use super::*;
use std::convert::TryInto;

pub trait RegexLexeme {
    fn needs_filtering() -> bool { false }
    fn except_from_these_words() -> Vec<&'static str> { vec![] }
    fn token_type() -> token::TokenType;
    fn expression() -> &'static str;
    fn recognize_raw_match(input : &str) -> Result<regex::Match, &'static str> {

        // early return, in case of invalid regex
        if let Err(e) = regex::Regex::new(Self::expression()) {
            return Err("Cannot compile regex.")
        }

        let matcher = regex::Regex::new(Self::expression()).unwrap();
        match matcher.find(input) {
            Some(position) => {
                if (position.start() > 0) {
                    Err("There is a match, but it is far away.")
                } else {
                    Ok (position)
                }
            },
            None => {
                Err("No match at all.")
            }
        }
    }
}

impl<T> lexeme::Lexeme for T
where T : RegexLexeme
{
    // using err here because it allows to express the reason for failing
    fn recognize(input : &str) -> Result<token::Token, &'static str> {
        if <Self as RegexLexeme>::needs_filtering() {
            let reserved_id = <Self as RegexLexeme>
                ::except_from_these_words();

            if let Ok(position) = <Self as regex_backend::RegexLexeme>
                ::recognize_raw_match(input) {

                if reserved_id.contains(&position.as_str()) {
                    Err("QVarId is not allowed to collide with reserved_id")
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
                Err("Filtering recognizer failed")
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
                    Err(reason)
                }
            }
        }
    }
}
