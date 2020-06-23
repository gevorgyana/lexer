use crate::dfa;

/// Pushdown finite state automata
pub trait PDA : dfa::DFA {
    type Stack;
}
