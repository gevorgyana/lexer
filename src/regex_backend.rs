use regex;
use super::*;
use std::convert::TryInto;

pub trait RegexLexeme {
    fn token_type() -> token::TokenType;
    fn expression() -> &'static str;
}

impl<T> lexeme::Lexeme for T
where
    T : RegexLexeme,
{
    // using err here because it allows to express the reason for failing
    fn recognize(input : &str) -> Result<token::Token, &'static str> {
        // todo instead of unwrapping, implement error handling
        let matcher = regex::Regex::new(Self::expression())
            .unwrap();

        match matcher.find(input) {
            Some(position) => {
                if (position.start() > 0) {
                    Err("")
                } else {
                    Ok (
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
            },
            None => {
                Err("")
            }
        }
    }
}
