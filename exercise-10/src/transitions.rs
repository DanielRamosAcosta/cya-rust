use crate::transition::Transition;

#[derive(Debug, Clone)]
pub struct Transitions {
    pub transitions: Vec<Transition>,
}

impl Transitions {
    pub fn new(transitions: Vec<Transition>) -> Transitions {
        Transitions { transitions }
    }

    pub fn find_transition_for(&self, symbol: char) -> Option<&Transition> {
        self.transitions.iter().find(|t| t.is_for(symbol))
    }
}
