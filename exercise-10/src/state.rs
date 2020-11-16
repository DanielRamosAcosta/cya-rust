use crate::transitions::Transitions;

pub struct State {
    id: u32,
    is_final: bool,
    transitions: Transitions,
}

impl State {
    pub fn new(
        id: u32,
        is_final: bool,
        transitions: Transitions,
    ) -> State {
        State {
            id,
            is_final,
            transitions,
        }
    }
}
