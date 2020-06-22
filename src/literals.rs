/// asterisk (*) from the left marks terminals
/// w/o{exclude} means exclusion
/// ---- Global rules ----
/// literal -> integer | float | char | string
///
/// --- Numeric literals ---
/// integer -> decimal | 0o octal | 0O octal
///          | 0x hexadecimal | 0X hexadecimal
/// float -> decimal . decimal [ exponent ]
///
/// --- Numeric literals helpers ---
/// decimal -> digit { digit }
/// octal -> octit { octit }
/// hexadecimal -> hexit { hexit }
/// exponent -> ( e | E ) [ + | - ] decimal

struct IntegerLiteral {}

impl RegexLexeme for IntegerLiteral {
    fn token_type() -> token::TokenType {
        token::TokenType::IntegerLiteral
    }

    fn expression() -> &'static str {
        r"([0-9]+|0o[0-7]+|0O[0-7]+|0x[0-9]+|0X[0-9]+)"
    }
}

struct FloatLiteral {}

impl RegexLexeme for FloatLiteral {
    fn token_type() -> token::TokenType {
        token::TokenType::FloatLiteral
    }

    fn expression() -> &'static str {
        r"[0-9]+\.[0-9]+((e|E)(+|-)?[0-9]+)?"
    }
}

/// --- Character literals ---
/// char -> ' ( graphic w/o{ ' | \ } | space | escape w/o{ \& } ) '
/// escape -> \ ( charsec | ascii | decimal | o octal | x hexadecimal )
/// *charsec* -> a | b | f | n | r | t | v | \ | " | ' | &
/// *ascii* -> ^ cntrl | NUL | SOH | STX | ETX | EOT | ENQ | ACK
///        | BEL | BS | HT | LF | VT | FF | CR | SO | SI | DLE
///        | DC1 | DC2 | DC3 | DC4 | NAK | SYN | ETB | CAN
///        | EM | SUB | ESC | FS | GS | RS | US | SP | DEL
/// cntr -> ascLarge | @ | [ | \ | ] | ^ | _
/// *ascLarge* -> A-Z
/// *whitechar* -> '\n' | '\v' | ' ' | '\t'
/// decimal -> digit { digit }
/// octal -> octit { octit }
/// *octit* -> [0-7]+
/// hexadecimal -> hexit { hexit }
/// *hexit* -> digit | A-Z
/// graphic -> [a-z] | [A-Z] | symbol | digit | special | " | '
/// *symbol* -> ! | # | $ | % | & | ⋆ | + | . | / | < | = | > | ? | @
///           | \ | ^ | | | - | ~ | :
/// *digit* -> [0-9]+
/// *special* -> ( | ) | , | ; | [ | ] | ` | { | }

struct CharLiteral {}

impl RegexLexeme for CharLiteral {
    fn token_type() -> token::TokenType {
        token::TokenType::CharLiteral
    }
    fn expression() -> &'static str {
        r"" // ascii
    }
}
