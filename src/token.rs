#[derive(PartialEq, PartialOrd, Debug)]
pub enum TokenType {
    Ident,
    MLComment,
    SLComment,
    ReservedId,
    ReservedOp,
    QConId,
    QVarId,
    QVarSym,
    QConSym,
    Special,
    IntegerLiteral,
    FloatLiteral,
    CharLiteral,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Token {
    pub span : Vec<u16>,
    pub token_type : TokenType,
}
