use crate::dfa;
use crate::ascii;
use crate::lexeme;
use crate::token;

use dfa::DFA;

/// Multiline comment DFA.

pub struct MLComment {
    state : MLCommentState,
}

impl MLComment {
    fn new() -> Self {
        Self {
            state : MLCommentState::Initial
        }
    }
}

#[derive(PartialEq)]
pub enum MLCommentState {
    Initial,
    SawOpeningBracket,
    SawOpenComm,
    SawDashAfterOpenComm,
    SawClosingBracket,
    Final,
    FailedMatch,
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
    fn advance(&mut self, input : Self::Input) {
        let input = input.get_char();
        match (&mut self.state, &input) {

            // special case
            (Self::State::FailedMatch, char) => {
                () // nothing happens, we have already failed
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
            },
            (Self::State::SawOpeningBracket, char) => {
                self.state = Self::State::FailedMatch;
            },

            // SawOpenComm -> *
            (Self::State::SawOpenComm, '-') => {
                self.state = Self::State::SawDashAfterOpenComm;
            },
            (Self::State::SawOpenComm, char) => {
                // the comment has started, we cannot fail given valid input
            },

            // SawDashafterOpenComm
            (Self::State::SawDashAfterOpenComm, '}') => {
                self.state = Self::State::Final;
            },
            (Self::State::SawOpenComm, char) => {
                // the comment has started, we cannot fail given valid input
            },

            _ => (),
        }
    }
}

impl lexeme::Lexeme for MLComment {

    fn recognize(input : &str) -> Option<token::Token> {
        let mut rec = MLComment::new();
        for character in input.chars() {
            match ascii::ASCIIChar::new(character) {
                Some(ascii_char) => { rec.advance(ascii_char) },
                None => { panic!("Non-ascii character found") },
            }
        }

        if rec.in_final_state() {
            Some(token::Token {
                token_type : token::TokenType::MLComment,
                span : token::Span::SingleLine(4),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::lexeme::Lexeme;

    #[test]
    fn lexeme() {
        assert_eq!(MLComment::recognize("{--}"),
                   Some (token::Token { token_type :
                                        token::TokenType::MLComment,
                                        span : token::Span::SingleLine(4),
                                        }));
    }

}
