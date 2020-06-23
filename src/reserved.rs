use crate::*;

pub struct ReservedId {}

impl regex::RegexLexeme for ReservedId {
    fn expression() -> &'static str {
        r"(case|class|data|default|deriving|do|else|foreign|if|import|in|infix|infixl|infixr|instance|let|module|newtype|of|then|type|where|_)"
    }

    fn token_type() -> token::TokenType {
        token::TokenType::ReservedId
    }
}

pub struct ReservedOp {}

impl regex::RegexLexeme for ReservedOp {
    fn expression() -> &'static str {
        // the last part is tricky! \\ is for \, and \| is for |, so the last
        // symbol in this string is not a delimiter!
        // update: important to escape the dots!
        r"\.\.|:|::|=|<-|->|@|~|=>|\\|\|"
    }

    fn token_type() -> token::TokenType {
        token::TokenType::ReservedOp
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexeme::Lexeme;

    /*
    #[test]
    fn lexeme_reserved_id() {
        let res = ReservedId::recognize("case").unwrap();
        assert_eq!(res.span, [4]);
    }

    #[test]
    fn lexeme_reserved_op() {
        let res = ReservedOp::recognize(r"\");//.unwrap();
        assert_eq!(res,
                   Ok(token::Token
                      { span : vec![1],
                        token_type : token::TokenType::ReservedOp}))
    }
     */
}
