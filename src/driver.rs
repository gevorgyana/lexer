use crate::*;
use crate::lexeme::Lexeme;

/// Generates Haskell token stream, uses a string view on the source code.
/// Operates on the top-level lexemes. These '__lexemes__' are not implemented.
/// ---- Rules covered at this scope ----
/// `program -> whitespace | lexeme`
///
/// `whitespace -> whitestuff {whitestuff}
/// `whitestuff -> whitechar | __comment__ | mlcomment
/// `whitechar -> '\n' | '\r' | ' ' | '\t'
///
/// `lexeme -> qvarid | qconid | qvarsym | qconsym
///          | __literal__ | special | reservedop | reservedid

fn gen_hs_token_stream(string_view : &str) -> Vec<token::Token> {
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

    // these are single characters, makes sense to check them here
    let special = vec!['|', ',', ';', '[', ']', '`', '{', '}'];

    let lexemes : Vec<fn(&str) -> Result::<token::Token, lexeme::Error>> = vec![
        // todo rethink the grammar one more time, esp. how identifiers exclude
        // reserved ids and ops - seems okay but ?
        mlcomment::MLComment::recognize,
        reserved::ReservedId::recognize,
        reserved::ReservedOp::recognize,
        qident::QConId::recognize,
        qident::QVarId::recognize,
        qident::QVarSym::recognize,
        qident::QConSym::recognize,
    ];

    while buffer_offset < string_view.len() {

        // whitespace-like (non-comment)
        if whitechar.contains(&string_view.chars().nth(buffer_offset).unwrap()) {
            buffer_offset += 1;
            continue
        }

        if special.contains(&string_view.chars().nth(buffer_offset).unwrap()) {
            token_stream.push(token::Token { span : vec![1],
                                             token_type : token::TokenType::Special
            });
            continue
        }

        for recognizer in &lexemes {
            if let Ok(token) = recognizer(&string_view[buffer_offset..]) {
                // todo maximal munch!
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

    // TODO need to test for EVERY LEXEME IN SEPARATION !!! THERE ARE BUGS ALREADY,
    // AND THERE WILL BE MORE

    /* #[test]
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
     */

    #[test]
    fn run() {
        let raw = std::format!(r"{modid}", modid = "value");
        assert_eq!(raw, "value");
    }
}
