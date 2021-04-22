use crate::state::State;

pub struct States {
    states: Vec<State>,
}

impl States {
    pub fn new(states: Vec<State>) -> States {
        States { states }
    }

    pub fn state_with_id(&self, id: u32) -> Option<State> {
        self.states
            .iter()
            .find(|s| s.has_id(id))
            .map(|s| s.clone() as State)
    }
}
