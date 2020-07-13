/// This file contiains terminals used for recognizing lexemes in a
/// Haskell program. They may be implemented with simple automata or
/// regular expressions that do the same thing under the hood. Nothing
/// more than regex is needed.

use super::regex;
use super::token;

struct BigASCII {}
impl regex::RegexLexeme for BigASCII {
    fn expression() -> &'static str { "A-Z" }
    fn token_type() -> token::TokenType { token::TokenType::BigASCII }
}

struct LowASCII {}
impl regex::RegexLexeme for LowASCII {
    fn expression() -> &'static str { "a-z" }
    fn token_type() -> token::TokenType { token::TokenType::LowASCII }
}

struct Digit {}
impl regex::RegexLexeme for Digit {
    fn expression() -> &'static str { "0-9" }
    fn token_type() -> token::TokenType { token::TokenType::Digit }
}

struct Octit {}
impl regex::RegexLexeme for Octit {
    fn expression() -> &'static str { "0-7" }
    fn token_type() -> token::TokenType { token::TokenType::Octit }
}

struct Hexit {}
impl regex::RegexLexeme for Hexit {
    fn expression() -> &'static str {
        static LAZY: ::lazy_static::lazy::Lazy<String> =
            ::lazy_static::lazy::Lazy::INIT;
        &*LAZY.get(|| { format!("A-Fa-f{}",
                                <Digit as regex::RegexLexeme>::expression()) })
    }
    fn token_type() -> token::TokenType { token::TokenType::Hexit }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::regex;
    #[test]
    fn foo() {
        // first test the dependant type
        assert_eq!("0-9", <Digit as regex::RegexLexeme>::expression());
        assert_eq!("[0-9]", <Digit as regex::CharacterGroup>::expression());
        assert_eq!(<Hexit as regex::RegexLexeme>::expression(), "A-Fa-f0-9");
        assert_eq!(<Hexit as regex::CharacterGroup>::expression(), "[A-Fa-f0-9]");
    }
}
