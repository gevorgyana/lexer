#[derive(PartialEq, PartialOrd, Debug)]
pub enum TokenType {
    Ident,
    MLComment,
    SLComment,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Span {
    // (takes this many characters)
    SingleLine(u8),
    // (takes this many characters, this many full lines,
    // and this many characters on the last line involved)
    Multiline(u8, u8, u8),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Token {
    pub span : Span,
    pub token_type : TokenType,
}
