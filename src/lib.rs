use std::rc::Rc;
use std::collections::HashMap;

/// Token types
enum TokenType {
    Ident,
}

/// ASCII char is the only type of char allowed
struct ASCIIChar {
    data : char,
}

/// And we have to be able to reject the chars that are not ASCII, hence
/// `optional`
impl ASCIIChar {
    fn new(data : char) -> Option<Self> {
        if data.is_ascii() {
            Option::Some(Self {data : data})
        } else {
            Option::None
        }
    }
}

/// DFA moves by accepting Input - this is very abstract
pub trait DFA {
    type Input;
    fn advance(&mut self);
    fn can_terminate(&self) -> bool;
}

/// But here is an example, a multiline comment DFA, first of all
/// it needs some state to know where it is at (transition_map)
struct DFAMLComment {
    transition_map : HashMap<u8, u8>,
}

/// And it moves accordingly
impl DFA for DFAMLComment {

    /// works with ASCII chars
    type Input = ASCIIChar;
    fn advance(&mut self) {

    }

    fn can_terminate(&self) -> bool {
        false
    }
}

fn test() {
    //let f = DFA {data  : 'f};
}
