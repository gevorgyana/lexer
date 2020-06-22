use crate::token;
use crate::lexeme;
use crate::regex_backend;

use std::convert::TryInto;
use crate::regex_backend::RegexLexeme;

pub struct QConId {}

impl regex_backend::SelfContained for QConId {}

pub struct QVarId {}

/// ---- Rules ----
/// qconid -> [modid .] conid
/// conid -> LARGE {SMALL | LARGE | DIGIT | '}
///
/// qvarid -> [modid .] varid
/// varid -> ( SMALL {SMALL | LARGE | DIGIT | '} ) / reservedid
///
/// modid -> {conid .} conid

impl regex_backend::RegexLexeme for QConId {
    fn expression() -> &'static str {
        r"(([A-Z][A-Za-z0-9']*\.)*[A-Z][A-Za-z0-9']*\.)?[A-Z][A-Za-z0-9']*"
    }
    fn token_type() -> token::TokenType {
        token::TokenType::QConId
    }
}

/// to make this lexeme self-contained, it is necessary to check if it
/// tried to lex a reserved keyword; it is not possible to express this
/// requirement in a regular experssion, therefore I chose to override
/// the provided blanket implementation for the Lexeme trait
impl regex_backend::RegexLexeme for QVarId {
    fn expression() -> &'static str {
        r"(([a-z][A-Za-z0-9']*\.)*[A-Z][A-Za-z0-9']*\.)?[A-Z][A-Za-z0-9']*"
    }
    fn token_type() -> token::TokenType {
        token::TokenType::QVarId
    }
}

impl lexeme::Lexeme for QVarId {
    fn recognize(input : &str) -> Result<token::Token, &'static str> {
        let reserved_id = vec!["case", "class", "data",
                               "default", "deriving", "do",
                               "else", "foreign", "if", "import",
                               "in", "infix", "infixl", "infixr",
                               "instance", "let", "module", "newtype",
                               "of", "then", "type", "where"];

        if let Ok(position) = <Self as regex_backend::RegexLexeme>::recognize_raw_match(input) {
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
            Err("Intercepted recognizer for QVarId failed")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexeme::Lexeme;

    #[test]
    fn qconid() {
        // todo remove unwraps, use Ok() and Err()
        let res = QConId::recognize("A.F").unwrap();
        assert_eq!(res.span, vec![3]);
        let res = QConId::recognize("A.F.f").unwrap();
        assert_eq!(res.span, vec![3]); // 3, not 5!
        let res = QConId::recognize(".");
        assert_eq!(res, Err("No match at all.")); // 3, not 5!
        let res = QConId::recognize("A'.F'.f").unwrap();
        assert_eq!(res.span, vec![5]);
        let res = QConId::recognize("Aa2'.F2f'.f22").unwrap();
        assert_eq!(res.span, vec![9]);

        // examples from the report (2.4 Identifiers and Operators)
        let res = QConId::recognize("f.g");
        assert_eq!(res, Err("No match at all."));
        let res = QConId::recognize("F.g").unwrap();
        assert_eq!(res.span, vec![1]); // F, g is small, so the
        // expression is not qconid!
        let res = QConId::recognize("f..");
        assert_eq!(res, Err("No match at all."));
        let res = QConId::recognize("F..").unwrap();
        assert_eq!(res.span, vec![1]); // qualified, but not qconid!
        // the same thind as with F.g, it is not qconid, but it would be
        // if g was G, like here
        let res = QConId::recognize("F.G").unwrap();
        assert_eq!(res.span, vec![3]); // perfectly valid qconid
        let res = QConId::recognize("F.").unwrap();
        assert_eq!(res.span, vec![1]); // It started as qconid, but no
    }
}
