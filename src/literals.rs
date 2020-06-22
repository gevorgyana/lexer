/// asterisk (*) from the left marks terminals
/// w/o{exclude} means exclusion
///
/// ---- Global rules ----
/// literal -> integer | float | char | string
///
/// --- Character literals ---
/// char -> ' ( graphic w/o{ ' | \ } | space | escape w/o{ \& } ) '
/// graphic -> [a-z] | [A-Z] | symbol | digit | special | " | '
/// escape -> \ ( charsec | ascii | decimal | o octal | x hexadecimal )
/// *charsec* -> a | b | f | n | r | t | v | \ | " | ' | &
/// *ascii* -> ^ cntrl | NUL | SOH | STX | ETX | EOT | ENQ | ACK
///        | BEL | BS | HT | LF | VT | FF | CR | SO | SI | DLE
///        | DC1 | DC2 | DC3 | DC4 | NAK | SYN | ETB | CAN
///        | EM | SUB | ESC | FS | GS | RS | US | SP | DEL
/// cntrl -> A-Z | @ | [ | \ | ] | ^ | _
/// decimal -> digit { digit }
/// octal -> octit { octit }
/// *octit* -> [0-7]
/// hexadecimal -> hexit { hexit }
/// *hexit* -> digit | A-F | a-f
/// *symbol* -> ! | # | $ | % | & | ⋆ | + | . | / | < | = | > | ? | @
///           | \ | ^ | | | - | ~ | :
/// *digit* -> [0-9]
/// *special* -> ( | ) | , | ; | [ | ] | ` | { | }

/// graphic
/// A-Z | a-z | 0-9
///   ( | ) | , | ; | [ | ] | ` | { | } |  ! | # | $ | % | & | ⋆ | + | . | / | < |
///   = | > | ? | @ | \ | ^ | | | - | ~ | : | |

/// escape
/// \
///   a | b | f | n | r | t | v | \ | " | ' | &
///   [0-9]+ | o [0-7]+ | x [A-Fa-f0-9]+
///   | NUL | SOH | STX | ETX | EOT | ENQ | ACK
///   | BEL | BS | HT | LF | VT | FF | CR | SO | SI | DLE
///   | DC1 | DC2 | DC3 | DC4 | NAK | SYN | ETB | CAN
///   | EM | SUB | ESC | FS | GS | RS | US | SP | DEL
///        ^
///           [A-Z] | @ | [ | \ | ] | ^ | _
///
/// gap
/// \ ('\n' | '\r\n' | '\r' |  '\v' | '\f' | ' ' | '\t')+ \
/// \ LF | CR LF | CR | VT | FF | SPACE | TAB
