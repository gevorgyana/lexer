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
    // trait is able to even define the type of State to be an integer. Idn how to
    // enforce this with Rust...
    fn advance(&mut self, input : Self::Input);
}

/// Multiline comment DFA.
/// It needs some state to know where it is at, the rules of transiton are defined
/// later in the advance() method
struct MLComment {
    state : <MLComment as DFA>::State,

    // Here is what I did earlier - the explicit table - of course this is not
    // acceptable

    /* transition_map : HashMap<(<MLComment as DFA>::Input, u8), u8>, */
}

/// Some enumeration for multiline comment automaton
enum MLCommentState {
    Initial,
    SawOpeningBracket,
    SawOpenComm,
    SawDashAfterOpenComm,
    SawClosingBracket,
    Final,
    FailedMatch, // see the comment about grep vs lex in README;
}

impl MLComment {
    fn new() -> Self {
        Self {
            state : <MLComment as DFA>::State::Initial
        }
    }
}

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
