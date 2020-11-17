use crate::transition::Transition;
use crate::transitions::Transitions;

#[derive(Clone)]
pub struct State {
    id: u32,
    is_final: bool,
    transitions: Transitions,
}

impl State {
    pub fn new(id: u32, is_final: bool, transitions: Transitions) -> State {
        State {
            id,
            is_final,
            transitions,
        }
    }

    pub fn find_transition_for(&self, symbol: char) -> Option<&Transition> {
        self.transitions.find_transition_for(symbol)
    }

    pub fn has_id(&self, id: u32) -> bool {
        self.id.eq(&id)
    }

    pub fn is_final_state(&self) -> bool {
        self.is_final
    }
}
