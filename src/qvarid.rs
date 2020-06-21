use crate::token;
use crate::lexeme;

use std::convert::TryInto;

use regex;

struct QVarId {}

/// ---- Rules ----
/// qvarid -> [modid .] conid
/// modid -> {conid .} conid
/// conid -> LARGE {SMALL | LARGE | DIGIT | '}

impl lexeme::Lexeme for QVarId {
    fn recognize(input : &str) -> Option<token::Token> {
        let matcher = regex::Regex::new(
            r"(([A-Z][A-Za-z0-9']*\.)*[A-Z][A-Za-z0-9']*\.)?[A-Z][A-Za-z0-9']*")
            .unwrap();

        match matcher.find(input) {
            Some(position) => {
                if (position.start() > 0) {
                    None
                } else {
                    Some (
                        token::Token {
                            span : vec![
                                (position.end() -
                                 position.start())
                                    .try_into()
                                    .unwrap()],
                            token_type : token::TokenType::QVarId,
                        }
                    )
                }
            },
            None => {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexeme::Lexeme;

    #[test]
    fn lexeme() {
        let res = QVarId::recognize("A.F").unwrap();
        assert_eq!(res.span, vec![3]);
        let res = QVarId::recognize("A.F.f").unwrap();
        assert_eq!(res.span, vec![3]); // 3, not 5!
        let res = QVarId::recognize(".");
        assert_eq!(res, None); // 3, not 5!
        let res = QVarId::recognize("A'.F'.f").unwrap();
        assert_eq!(res.span, vec![5]);
        let res = QVarId::recognize("Aa2'.F2f'.f22").unwrap();
        assert_eq!(res.span, vec![9]);

        // examples from the report (2.4 Identifiers and Operators)
        let res = QVarId::recognize("f.g");
        assert_eq!(res, None);
        let res = QVarId::recognize("F.g").unwrap();
        assert_eq!(res.span, vec![1]); // F, g is small, so the
        // expression is not qvarid!
        let res = QVarId::recognize("f..");
        assert_eq!(res, None);
        let res = QVarId::recognize("F..").unwrap();
        assert_eq!(res.span, vec![1]); // qualified, but not qvarid!
        // the same thind as with F.g, it is not qvarid, but it would be
        // if g was G, like here
        let res = QVarId::recognize("F.G").unwrap();
        assert_eq!(res.span, vec![3]); // perfectly valid varid
        let res = QVarId::recognize("F.").unwrap();
        assert_eq!(res.span, vec![1]); // It started as varid, but no
    }
}
