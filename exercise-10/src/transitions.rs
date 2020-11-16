use crate::transition::Transition;

#[derive(Debug)]
pub struct Transitions {
    pub transitions: Vec<Transition>
}

impl Transitions {
    pub fn new(transitions: Vec<Transition>) -> Transitions {
        Transitions { transitions }
    }
}
