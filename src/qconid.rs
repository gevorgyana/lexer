use crate::token;
use crate::lexeme;
use crate::regex_backend;

pub struct QConId {}

/// ---- Rules ----
/// qvarid -> [modid .] conid
/// modid -> {conid .} conid
/// conid -> LARGE {SMALL | LARGE | DIGIT | '}

impl regex_backend::RegexLexeme for QConId {
    fn expression() -> &'static str {
        r"(([A-Z][A-Za-z0-9']*\.)*[A-Z][A-Za-z0-9']*\.)?[A-Z][A-Za-z0-9']*"
    }
    fn token_type() -> token::TokenType {
        token::TokenType::QConId
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::lexeme::Lexeme;

    #[test]
    fn lexeme() {
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
