use std::rc::Rc;
use std::collections::HashMap;
use std::fs;

enum TokenType {
    Ident,
}

/// ASCII char, the only type of char allowed, by the way
#[derive(Eq, Hash, PartialEq)]
struct ASCIIChar {
    data : char,
}

/// Be able to reject the chars that are not ASCII, hence `optional`
impl ASCIIChar {
    fn new(data : char) -> Option<Self> {
        if data.is_ascii() {
            Option::Some(Self {data : data} )
        } else {
            Option::None
        }
    }
}

/// This is very abstract - only forces the implementors to define the two types
/// and one method (there is no restriction on types unfortunately)
pub trait DFA {
    type State;
    type Input;
    // user code should keep calling this until the state is Failed or Final;
    // this is a convertion, not eforced by the trait-impl relationship, in
    // general, there is no way to force the State to be a enum that has some
    // predefined values (Failed, Final, Initial), any object that implements this
    // trait is able to even define the type on State to be an integer. Idn how to
    // enforce this with Rust...
    fn advance(&mut self, input : Self::Input);
}

/// But here is an example, a multiline comment DFA.
/// First of all it needs some state to know where it is at and the rules to
/// move forward - UPDATE - the rules are minimal, they are implemented in
/// the advance() function, not hardcoded explicitly!!!
struct MLComment {
    state : <MLComment as DFA>::State,

    // DO NOT DO THIS; an example of a huge waste of memory!
    // (input, state) -> state
    // transition_map : HashMap<(<MLComment as DFA>::Input, u8), u8>,
}

// UPDATE! The dumbest thing to do is to explicitly define a lot of
// transitions in a table; that is ugly and wastes memory; I can minimize the
// automaton and have only a couple of states (I still need then though, for any
// case where there are more than 1 character in a successful match) [1]!

// UPDATE! Formally, the automata is not minimized [1], but in practice it is either
// checking at runtime, or wasting memory; a small runtime check is not that bad.

/// Some enumeration for multiline comment automaton
enum MLCommentState {
    Initial,
    SawOpeningBracket,
    SawOpenComm,
    SawDashAfterOpenComm,
    SawClosingBracket,
    Final,
    FailedMatch, // see the comment about grep vs lex; a match is failed at position i
    // if there is no token of current type starting from i-th char in the input text
    // other words, it is about automata that recognize token starting AT SOME PLACE,
    // NOT the ones that look for them all over the source code
}

impl MLComment {
    fn new() -> Self {
        Self {
            state : <MLComment as DFA>::State::Initial
        }
    }
}

/// IMPORTANT; how the lexer works; the main thing to notice here is that a DFA
/// can in princtiple be used on the whole inupt stream, for example, like this:
/// {/ dfdfdfdf {- -}
/// here, the {/ is not matched against the beginning of the block of multiline
/// comment, but initially I wanted to continue from that point, other words, I
/// would move to the initial state and wait until {- arrives, then i would move
/// to the state when one part of the pattern has matched, and finish when i saw
/// -}; THIS IS HOW GREP WORKS! NOT HOW LEX WORKS! I NEED LEX-LIKE BEHAVIOUR

impl DFA for MLComment {

    /// works with ASCII chars
    type Input = ASCIIChar;
    /// and has a custom enum for describing the state
    type State = MLCommentState;

    // this implicitly implements the transition table as all DFAs do
    fn advance(&mut self, input : Self::Input) {
        match (&mut self.state, &input) {

            // special case
            (Self::State::FailedMatch, ASCIIChar {data}) => {
                () // nothing happens, we have already failed
            },

            // Initial -> *
            (Self::State::Initial, ASCIIChar {data : '{'}) => {
                self.state = Self::State::SawOpeningBracket;
            },
            (Self::State::Initial, ASCIIChar {data}) => {
                self.state = Self::State::FailedMatch;
            },

            // SawOpeningBracket -> *
            (Self::State::SawOpeningBracket, ASCIIChar {data : '-'}) => {
                self.state = Self::State::SawOpenComm;
            },
            (Self::State::SawOpeningBracket, ASCIIChar {data}) => {
                self.state = Self::State::FailedMatch;
            },

            // SawOpenComm -> *
            (Self::State::SawOpenComm, ASCIIChar {data : '-'}) => {
                self.state = Self::State::SawDashAfterOpenComm;
            },
            (Self::State::SawOpenComm, ASCIIChar {data}) => {
                // the comment has started, we cannot fail given valid input
            },

            // SawDashafterOpenComm
            (Self::State::SawDashAfterOpenComm, ASCIIChar {data : '}'}) => {
                self.state = Self::State::Final;
            },
            (Self::State::SawOpenComm, ASCIIChar {data}) => {
                // the comment has started, we cannot fail given valid input
            },

            _ => (),
        }
    }
}

fn test() {
    // read the whole file into a string and move on from there
    fs::read_to_string("test.hs");
}
