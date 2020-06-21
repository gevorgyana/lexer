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

    fn recognize(input : &str) -> Result<token::Token, &'static str> {
        let mut rec = MLComment::new();
        let mut cols_in_curr_line = 0;
        let mut lines_span : Vec<u16> = vec![];

        for character in input.chars() {
            match ascii::ASCIIChar::new(character) {
                Some(ascii_char) => {
                    match ascii_char.get_char() {
                        '\n' => {
                            lines_span.push(cols_in_curr_line);
                            cols_in_curr_line = 0;
                        },
                        _ => {
                            cols_in_curr_line += 1;
                        },
                    }

                    // if it is in a final state but there is still more
                    // to see? should not happen when the stack is 0, so what
                    // is after another opening bracket is another token

                    rec.advance(ascii_char);
                    if rec.in_final_state() {

                        // do not forget the last line
                        lines_span.push(cols_in_curr_line);

                        return Ok(token::Token
                                    { token_type :
                                      token::TokenType::MLComment,
                                      span : lines_span,
                                    }) // ! -> ()
                    } else if rec.in_fail_state() {
                        return Err("") // ! -> ()
                    }
                },
                None => { panic!("Non-ascii character found") },
            }
        }

        // no token has been recognized
        Err("")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::lexeme::Lexeme;

    #[test]
    fn lexeme() {
        assert_eq!(MLComment::recognize("{--}"),
                   Ok (
                       token::Token
                       { token_type :
                         token::TokenType::MLComment,
                         span : vec![4],
                       }));

        assert_eq!(MLComment::recognize("{-dfasdfasdf-}"),
                   Ok (
                       token::Token
                       { token_type :
                         token::TokenType::MLComment,
                         span : vec![14],
                       }));

        assert_eq!(MLComment::recognize("{-{--}-}"),
                   Ok (
                       token::Token
                       { token_type :
                         token::TokenType::MLComment,
                         span : vec![8],
                       }));


        assert_eq!(MLComment::recognize("{-\n{--}-}"),
                   Ok (
                       token::Token
                       { token_type :
                         token::TokenType::MLComment,
                         span : vec![2, 6],
                       }));

    }
}
