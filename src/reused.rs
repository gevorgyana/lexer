/// This file contiains terminals used for
/// recognizing lexemes in a Haskell program.
/// They may be implemented with simple automata
/// or regular expressions that do the same thing
/// under the hood. Nothing more than regex is needed.

use super::regex;
use super::token;

/// Big ASCII letters
struct BigASCII {}

impl regex::RegexLexeme for BigASCII {
    fn expression() -> &'static str {
        "A-Z"
    }

    fn token_type() -> token::TokenType {
        token::TokenType::BigASCII
    }
}

struct LowASCII {}

impl regex::RegexLexeme for LowASCII {
    fn expression() -> &'static str {
        "a-z"
    }

    fn token_type() -> token::TokenType {
        token::TokenType::LowASCII
    }
}

struct Digit {}

impl regex::RegexLexeme for Digit {
    fn expression() -> &'static str {
        "0-9"
    }

    fn token_type() -> token::TokenType {
        token::TokenType::Digit
    }
}

struct Octit {}

impl regex::RegexLexeme for Octit {

    fn expression() -> &'static str {
        "0-7"
    }

    fn token_type() -> token::TokenType {
        token::TokenType::Octit
    }
}

struct Hexit {}

impl regex::RegexLexeme for Hexit {

    fn expression() -> &'static str {
        /*
               lazy_static! {
                   static ref expr : String = std::format!("[A-Fa-f{digit}]",
                                                           digit = <Digit
                                                           as regex::RegexLexeme>
                                                           ::expression());
         */
        "sf"

        // *expr evaluates to the static String value, then & takes a view on it,
        // the whole expression is calculated once during the whole runtime, and
        // this is brilliant! see this thread
        // https://users.rust-lang.org/t/how-to-avoid-recalculating-a-formatted-string-at-runtime/44895/7

        //&*expr
    }

    fn token_type() -> token::TokenType {
        token::TokenType::Hexit
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn foo() {
        assert_eq!(Hexit::prepare(), "A-Fa-f0-9");
    }
}
