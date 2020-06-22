pub trait DFA {

    type State;

    // some automata work only with ASCII characters todo why?
    type Input;

    fn advance(&mut self, input : Self::Input);

    fn in_final_state(&self) -> bool;

    fn in_fail_state(&self) -> bool;
}

pub enum DFALexemeErr {
 //   DFA
}
