/// This file contiains terminals used for recognizing lexemes in a Haskell program.
/// They may be implemented with simple automata or regular expressions that do the
/// same thing under the hood. Nothing more than regex is needed.

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
    fn expression() -> &'static str { format!("[A-Fa-f{}]", "value") }
    fn token_type() -> token::TokenType { token::TokenType::Hexit }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn foo() {
        //assert_eq!(Hexit::prepare(), "A-Fa-f0-9");
    }
}
