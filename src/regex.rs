use super::lexeme;
use super::token;

use std::collections::HashMap;
use std::sync::Mutex;
use std::any::TypeId;
use once_cell::sync::OnceCell;

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

    /// Activates the usage of except_for() filtering, false by
    /// default.
    fn needs_filtering() -> bool { false }

    /// A regular expression that defines the matches that must be
    /// avoided. NOOP by default.
    fn except_for() -> &'static str { "" }

    fn token_type() -> token::TokenType;

    /// A regular expression that defines the matches that should be
    /// reported. In case character group is used, do not surround the
    /// expression with [], instead implement CharGroup for your type.
    fn expression() -> &'static str;

    /// For internal use, this function deals with regex implementation
    /// details.
    fn recognize_raw_match(input : &str) ->
        Result<regex_backend::Match, Error> {
        match regex_backend::Regex::new(Self::expression()) {
            Err(e) => Err(Error::InvalidExpression),
            Ok(matcher) => {
                match matcher.find(input) {
                    Some(position) => {
                        if (position.start() > 0)
                        { Err(Error::DistantMatch) }
                        else { Ok(position) }
                    },
                    None => { Err(Error::NoMatch) }
                }
            }
        }
    }
}

pub trait CharacterGroup : RegexLexeme
    where Self : 'static
{

    /// Wraps an inner expression into a pair of square brackets '[]'.
    /// Calculates the internal string that is returned by `format!`
    /// only once, returns a view on it that has static lifetime.
    fn expression() -> &'static str {
        static value : OnceCell<
                Mutex<HashMap<TypeId, &'static str>>>
            = OnceCell::new();

        let _: &'static Mutex<HashMap<TypeId, &'static str>> =
            value.get_or_init(|| {
                Mutex::new(HashMap::new())
            });

        let v: &'static str =
            value
            .get()
            .unwrap()
            .lock()
            .unwrap()
            .entry(TypeId::of::<Self>())
            .or_insert_with(|| {
                let string =
                    format!("[{}]",
                            <Self as RegexLexeme>::expression());
                Box::leak(string.into_boxed_str())
            });
        v
    }
}

impl <T : RegexLexeme + 'static> CharacterGroup for T {}

/// TODO Find a way to implement Maybe monad. The general pattern is
/// `try to compute; if not possible, wrap the internal error in this
/// more general enum, and return the result`. Nested ifs become
/// clumsy.

impl<T> lexeme::Lexeme for T
where T : RegexLexeme
{

    /// Wraps the general Lexeme trait for all RegexLexemes;
    /// - serves the purpose of error handling
    /// - optionally performs filtering with RegexLexeme::except_for()
    fn recognize(input : &str) -> Result<token::Token, lexeme::Error> {

        if <Self as RegexLexeme>::needs_filtering() {
            let except_for = <Self as RegexLexeme>::except_for();
            if let Ok(position) = <Self as RegexLexeme>::recognize_raw_match(input) {
                // todo this needs to be changed
                if except_for.contains(&position.as_str()) {
                    Err(lexeme::Error::FoundConflictingLexeme)
                } else {
                    Ok(token::Token { span : vec![ (position.end() - position.start())
                                                   .try_into().unwrap() ],
                                      token_type : Self::token_type(),
                    })
                }
            } else {
                Err(lexeme::Error::Regex(Error::NoMatch))
            }
        } else {
            match <Self as RegexLexeme>::recognize_raw_match(input) {
                Ok(position) => {
                    Ok(token::Token { span : vec![(position.end() - position.start())
                                                  .try_into().unwrap()],
                                      token_type : Self::token_type(),
                    })
                },
                Err(reason) => {
                    Err(lexeme::Error::Regex(reason))
                }
            }
        }
    }
}
