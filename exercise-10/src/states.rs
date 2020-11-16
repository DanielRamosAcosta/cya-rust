use crate::state::State;

pub struct States {
    states: Vec<State>,
}

impl States {
    pub fn new(states: Vec<State>) -> States {
        States { states }
    }
}
