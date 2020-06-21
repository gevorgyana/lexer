use crate::dfa;
use crate::pda;
use crate::ascii;
use crate::lexeme;
use crate::token;

use dfa::DFA;

/// Multiline comment DFA.

pub struct MLComment {
    state : MLCommentState,
    stack : u8,
}

impl MLComment {
    fn new() -> Self {
        Self {
            state : MLCommentState::Initial,
            stack : 0,
        }
    }
}

#[derive(PartialEq)]
pub enum MLCommentState {
    Initial,
    SawOpeningBracket,
    SawOpenComm,
    SawDashAfterOpenComm,
    //SawClosingBracket,
    SawOpeningBracketInner,
    Final,
    FailedMatch,
}

impl pda::PDA for MLComment {
    type Stack = u8;
}

impl dfa::DFA for MLComment {

    /// works with ASCII chars
    type Input = ascii::ASCIIChar;

    /// and has a custom enum for describing the states
    type State = MLCommentState;

    fn in_final_state(&self) -> bool {
        self.state == Self::State::Final
    }

    fn in_fail_state(&self) -> bool {
        self.state == Self::State::FailedMatch
    }

    // this implicitly implements the transition table as all DFAs do
    // and uses a stack for detecting correct bracket sequences
    fn advance(&mut self, input : Self::Input) {
        let input = input.get_char();
        match (&mut self.state, &input) {

            // Final/Failed
            (Self::State::Final, char) => {
                ()
            }
            (Self::State::FailedMatch, char) => {
                ()
            },

            // Initial -> *
            (Self::State::Initial, '{') => {
                self.state = Self::State::SawOpeningBracket;
            },
            (Self::State::Initial, char) => {
                self.state = Self::State::FailedMatch;
            },

            // SawOpeningBracket -> *
            (Self::State::SawOpeningBracket, '-') => {
                self.state = Self::State::SawOpenComm;
                self.stack += 1;
            },
            (Self::State::SawOpeningBracket, char) => {
                self.state = Self::State::FailedMatch;
            },

            // SawOpenComm -> *
            (Self::State::SawOpenComm, '-') => {
                self.state = Self::State::SawDashAfterOpenComm;
            },
            (Self::State::SawOpenComm, '{') => {
                self.state = Self::State::SawOpeningBracketInner;
            },
            (Self::State::SawOpenComm, char) => {},

            // SawDashAfterOpenComm -> *
            (Self::State::SawDashAfterOpenComm, '}') => {
                self.stack -= 1;
                if self.stack == 0 {
                    self.state = Self::State::Final;
                } else {
                    self.state = Self::State::SawOpenComm;
                }
            },
            (Self::State::SawDashAfterOpenComm, char) => {
                self.state = Self::State::SawOpenComm;
            },

            // SawOpeningBracketInner -> *
            (Self::State::SawOpeningBracketInner, '-') => {
                self.stack += 1;
                self.state = Self::State::SawOpenComm;
            },
            (Self::State::SawOpeningBracketInner, char) => {
                self.state = Self::State::SawOpenComm;
            },
        }
    }
}

impl lexeme::Lexeme for MLComment {

    fn recognize(input : &str) -> Option<token::Token> {
        let mut rec = MLComment::new();
        let mut nlines = 0;
        let mut beg_cols = 0;
        let mut end_cols = 0;

        for character in input.chars() {
            match ascii::ASCIIChar::new(character) {
                Some(ascii_char) => {
                    match ascii_char.get_char() {
                        '\n' => {
                            nlines += 1;
                            end_cols = 0
                        }
                        _ => {
                            if (nlines > 0) {
                                end_cols += 1
                            } else {
                                beg_cols += 1
                            }
                        }
                    }
                    rec.advance(ascii_char);
                    if rec.in_final_state() {
                        let span : token::Span;
                        if (nlines > 0) {
                            span = token::Span::Multiline(beg_cols, nlines, end_cols);
                        } else {
                            span = token::Span::SingleLine(beg_cols);
                        }

                        return Some(token::Token
                                    { token_type :
                                      token::TokenType::MLComment,
                                      span : span,
                                    }) // ! -> ()
                    } else if rec.in_fail_state() {
                        return None // ! -> ()
                    }
                },
                None => { panic!("Non-ascii character found") },
            }
        }

        // no token has been recognized
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::lexeme::Lexeme;

    #[test]
    fn lexeme() {
        assert_eq!(MLComment::recognize("{--}"),
                   Some (
                       token::Token
                       { token_type :
                         token::TokenType::MLComment,
                         span : token::Span::SingleLine(4),
                       }));

        assert_eq!(MLComment::recognize("{-dfasdfasdf-}"),
                   Some (
                       token::Token
                       { token_type :
                         token::TokenType::MLComment,
                         span : token::Span::SingleLine(14),
                       }));

        assert_eq!(MLComment::recognize("{-{--}-}"),
                   Some (
                       token::Token
                       { token_type :
                         token::TokenType::MLComment,
                         span : token::Span::SingleLine(8),
                       }));
    }
}
