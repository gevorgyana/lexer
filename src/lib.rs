use std::rc::Rc;
use std::collections::HashMap;

/// Token types
enum TokenType {
    Ident,
}

/// ASCII char, the only type of char allowed, by the way
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
/// First of all t needs some state to know where it is at (transition_map)
struct DFAMLComment {
    transition_map : HashMap<u8, u8>,
}

/// This is where we hardwire the logic of the automaton
impl DFAMLComment {
    fn new() -> Self {
        Self {transition_map : HashMap::new() }
    }
}

/// So that it moves accordingly to the transition map
impl DFA for DFAMLComment {

    /// works with ASCII chars
    type Input = ASCIIChar;

    fn advance(&mut self, input : Self::Input) {

    }

    fn can_terminate(&self) -> bool {
        false
    }
}
