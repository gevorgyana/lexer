use crate::*;
use crate::lexeme::Lexeme;

/// Generates Haskell token stream, uses a string view on the source

fn gen_hs_token_stream(string_view : &str) {
    // the value returned
    let mut token_stream : Vec<token::Token> = vec![];
    // how many characters away from the beginning of file
    let mut advanced_chars : usize = 0;

    // current position in the source buffer
    let mut cur_row : u32 = 0;
    let mut cur_col : u16 = 0;

    while advanced_chars < string_view.len() {

        // for every lexeme, try to recognize it
        // problem : some lexemes depend on the order of recognition

        if let Some(token) = mlcomment::MLComment::recognize(
            &string_view[advanced_chars..]) {
            advanced_chars += token.span.iter()
                .fold(token.span.len() - 1, |sum, x| sum + *x as usize);

            cur_row += token.span.len() as u32;
            cur_col = *token.span.last().unwrap();
            token_stream.push(token);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn run() {
        gen_hs_token_stream("dfsfdf");
    }
}
