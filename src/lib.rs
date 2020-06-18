use std::rc::Rc;
use std::collections::HashMap;
use std::fs;

/// Token types
enum TokenType {
    Ident,
}

/// ASCII char, the only type of char allowed, by the way
#[derive(Eq, Hash, PartialEq)]
struct ASCIIChar {
    data : char,
}

/// And we have to be able to reject the chars that are not ASCII, hence
/// `optional`
impl ASCIIChar {
    fn new(data : char) -> Option<Self> {
        if data.is_ascii() {
            Option::Some(Self {data : data} )
        } else {
            Option::None
        }
    }
}

/// DFA moves by accepting Input - this is very abstract
pub trait DFA {
    type Input;
    fn advance(&mut self, input : Self::Input);
    fn can_terminate(&self) -> bool;
}

/// But here is an example, a multiline comment DFA.
/// First of all it needs some state to know where it is at and the rules to
/// move forward - UPDATE - the rules are minimal, they are implemented in
/// the advance() function, not hardcoded explicitly!!!
struct DFAMLComment {
    state : u8,

    // DO NOT DO THIS; an example of a huge waste of memory!
    // (input, state) -> state
    // transition_map : HashMap<(<DFAMLComment as DFA>::Input, u8), u8>,
}

// UPDATE! The dumbest thing to do is to explicitly define a lot of
// transitions in a table; that is ugly and wastes memory; I can minimize the
// automaton and have only a couple of states (I still need then though, for any
// case where there are more than 1 character in a successful match) [1]!

// UPDATE! Formally, the automata is not minimized [1], but in practice it is either
// checking at runtime, or wasting memory; a small runtime check is not that bad

impl DFAMLComment {
    fn new() -> Self {
        Self {
            state : 0
        }
    }
}

impl DFA for DFAMLComment {

    /// works with ASCII chars
    type Input = ASCIIChar;

    fn advance(&mut self, input : Self::Input) {

    }

    fn can_terminate(&self) -> bool {
        false
    }
}

fn test() {
    // read the whole file into a string and move on from there
    fs::read_to_string("test.hs");
}
