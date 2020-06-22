use crate::*;
use crate::lexeme::Lexeme;

/// Generates Haskell token stream, uses a string view on the source code.
/// Operates on the top-level lexemes. These '__lexemes__' are handled at
/// other levels.
/// ---- Rules covered at this scope ----
/// `program -> whitespace | lexeme`
///
/// `whitespace -> whitestuff {whitestuff}
/// `whitestuff -> whitechar | __comment__ | mlcomment
/// `whitechar -> '\n' | '\r' | ' ' | '\t'
///
/// `lexeme -> __qvarid__ | qconid | __qvarsym__ | __qconsym__
///          | __literal__ | __special__ | reservedop | reservedid

fn gen_hs_token_stream(string_view : &str) -> Vec<token::Token>{
    // return value
    let mut token_stream : Vec<token::Token> = vec![];
    // how many characters away from the beginning of file
    let mut buffer_offset : usize = 0;

    // current position in the source buffer
    let mut cur_row : u32 = 0;
    let mut cur_col : u16 = 0;

    // only handle basic whitestuff, the 2010 report names more characters (vertical
    // tab, form feed, any unicode char that represents whitespace)
    let whitechar = vec!['\n', '\r', '\t', ' '];

    let lexemes : Vec<fn(&str) -> Result::<token::Token, &'static str>> = vec![
        // todo rethink the grammar one more time, esp. how identifiers exclude
        // reserved ids and ops
        mlcomment::MLComment::recognize,
        reserved::ReservedId::recognize,
        reserved::ReservedOp::recognize,
        qconid::QConId::recognize,
    ];

    while buffer_offset < string_view.len() {

        // whitespace-like (non-comment)
        if whitechar.contains(&string_view.chars().nth(buffer_offset).unwrap()) {
            buffer_offset += 1;
            continue
        }

        for recognizer in &lexemes {
            if let Ok(token) = recognizer(&string_view[buffer_offset..]) {
                // todo no cast?
                buffer_offset += token.span.iter()
                    .fold(token.span.len() - 1, |sum, x| sum + *x as usize);
                cur_row += token.span.len() as u32;
                cur_col = *token.span.last().unwrap();
                token_stream.push(token)
            } else {
                println!("No token recognized")
            }
        }
    }

    token_stream
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn mlcomment() {
        assert_eq!(gen_hs_token_stream("{--}"), vec![
            token::Token
            {
                span : vec![4],
                token_type : token::TokenType::MLComment
            }]);
    }
    #[test]
    fn qconid() {
        assert_eq!(gen_hs_token_stream("F.F"), vec![
            token::Token
            {
                span : vec![3],
                token_type : token::TokenType::QConId,
            }]);
    }
}
