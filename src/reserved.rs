use crate::*;

struct ReservedId {}

impl regex_backend::RegexLexeme for ReservedId {
    fn expression() -> &'static str {
        r"case|class|data|default|deriving|do|else|foreign|if|import|in|infix|infixl|infixr|instance|let|module|newtype|of|then|type|where|_"
    }

    fn token_type() -> token::TokenType {
        token::TokenType::ReservedId
    }
}

struct ReservedOp {}

impl regex_backend::RegexLexeme for ReservedOp {
    fn expression() -> &'static str {
        r"..|:|::|=|\|\||<-|->|@|~|=>"
    }

    fn token_type() -> token::TokenType {
        token::TokenType::ReservedOp
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexeme::Lexeme;

    #[test]
    fn lexeme_reserved_id() {
        let res = ReservedId::recognize("case").unwrap();
        assert_eq!(res.span, [4]);
    }

    #[test]
    fn lexeme_reserved_op() {
        let res = ReservedOp::recognize(r"\");//.unwrap();
        assert_eq!(res, Err(""));
        //assert_eq!(res.span, [1]);
    }
}
