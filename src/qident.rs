use crate::token;
use crate::lexeme;
use crate::regex;

use std::convert::TryInto;
use crate::regex::RegexLexeme;

/// ---- Rules ----
/// qconid -> [modid .] conid
/// conid -> LARGE {SMALL | LARGE | DIGIT | '}
///
/// qvarid -> [modid .] varid
/// varid -> ( SMALL {SMALL | LARGE | DIGIT | '} ) / reservedid
///
/// qvarsym -> [modid .] varsym
/// varsym -> ( ( symbol ) / ':' { symbol } ) / ( reservedop | dashes )
///
/// modid -> {conid .} conid

pub struct QConId {}

/// DONE
impl regex::RegexLexeme for QConId {

    fn expression() -> &'static str {
        r"(([A-Z][A-Za-z0-9']*\.)*[A-Z][A-Za-z0-9']*\.)?[A-Z][A-Za-z0-9']*"
    }
    fn token_type() -> token::TokenType {
        token::TokenType::QConId
    }
}

pub struct QVarId {}

/// DONE
impl regex::RegexLexeme for QVarId {

    fn needs_filtering() -> bool { true }

    fn except_for() -> &'static str {
        /*
        vec!["case", "class", "data",
             "default", "deriving", "do",
             "else", "foreign", "if", "import",
             "in", "infix", "infixl", "infixr",
             "instance", "let", "module", "newtype",
             "of", "then", "type", "where"]
         */
        r""
    }

    fn expression() -> &'static str {
        r"(([A-Z][A-Za-z0-9']*\.)*[A-Z][A-Za-z0-9']*\.)?[a-z][A-Za-z0-9']*"
    }

    fn token_type() -> token::TokenType {
        token::TokenType::QVarId
    }
}

pub struct QVarSym {}

impl regex::RegexLexeme for QVarSym {

    fn needs_filtering() -> bool { true }

    fn except_for() -> &'static str {
        /*
        vec![
            r"--", // this is a dirty trick, I need a regex for representing dashes
            r"..", r":", r"::", r"=", r"\", r"|", r"<-", r"->", r"@", r"~", r"=>"
        ]
         */
        r""
    }

    fn expression() -> &'static str {
        r"(([A-Z][A-Za-z0-9']*\.)*[A-Z][A-Za-z0-9']*\.)?([!|#|$|%|&|*|+|\.|/|<|=|>|?|@|\|^|\||-|~])?([!|#|$|%|&|*|+|\.|/|<|=|>|?|@|\|^|\||-|~|:])*"
    }

    fn token_type() -> token::TokenType {
        token::TokenType::QVarSym
    }
}

pub struct QConSym {}

impl regex::RegexLexeme for QConSym {

    fn needs_filtering() -> bool { true }

    fn except_for() -> &'static str {
        /*
        vec![
            r"--", // this is a dirty trick, I need a regex for representing dashes
            r"..", r":", r"::", r"=", r"\", r"|", r"<-", r"->", r"@", r"~", r"=>"
        ]
         */
        r""
    }

    fn expression() -> &'static str {
        r"(([A-Z][A-Za-z0-9']*\.)*[A-Z][A-Za-z0-9']*\.)?:([!|#|$|%|&|*|+|\.|/|<|=| > |?|@|\|^|\||-|~|:])*"
    }

    fn token_type() -> token::TokenType {
        token::TokenType::QConSym
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
        assert_eq!(res, Err (lexeme::LexemeErr::RegexErr
                             (regex::RegexLexemeErr::NoMatch)));
        let res = QConId::recognize("A'.F'.f").unwrap();
        assert_eq!(res.span, vec![5]);
        let res = QConId::recognize("Aa2'.F2f'.f22").unwrap();
        assert_eq!(res.span, vec![9]);

        // examples from the report (2.4 Identifiers and Operators)
        let res = QConId::recognize("f.g");
        assert_eq!(res, Err (lexeme::LexemeErr::RegexErr
                             (regex::RegexLexemeErr::NoMatch)));
        let res = QConId::recognize("F.g").unwrap();
        assert_eq!(res.span, vec![1]); // F, g is small, so the
        // expression is not qconid!
        let res = QConId::recognize("f..");
        assert_eq!(res, Err (lexeme::LexemeErr::RegexErr
                             (regex::RegexLexemeErr::NoMatch)));
        let res = QConId::recognize("F..").unwrap();
        assert_eq!(res.span, vec![1]); // qualified, but not qconid!
        // the same thind as with F.g, it is not qconid, but it would be
        // if g was G, like here
        let res = QConId::recognize("F.G").unwrap();
        assert_eq!(res.span, vec![3]); // perfectly valid qconid
        let res = QConId::recognize("F.").unwrap();
        assert_eq!(res.span, vec![1]); // It started as qconid, but no
    }

    #[test]
    fn qvarid() {
        let res = QVarId::recognize("f.g");

        //assert_eq!(res, Err("Intercepted recognizer for QVarId failed"));
        //let res = QVarId::recognize("F.g").unwrap();
        //assert_eq!(res.span, vec![0]); // F, g is small, so the
        // expression is not qconid!
        let res = QVarId::recognize("f..");
        //assert_eq!(res, Err("No match at all."));
        //let res = QVarId::recognize("F..").unwrap();
        //assert_eq!(res.span, vec![1]); // qualified, but not qconid!
        // the same thind as with F.g, it is not qconid, but it would be
        // if g was G, like here
        //let res = QVarId::recognize("F.G").unwrap();
        //assert_eq!(res.span, vec![3]); // perfectly valid qconid
        //let res = QVarId::recognize("F.").unwrap();
        //assert_eq!(res.span, vec![1]); // It started as qconid, but no
    }

    #[test]
    fn qvarsym() {

    }
}
