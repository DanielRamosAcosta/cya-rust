use crate::transition::Transition;
use crate::transitions::Transitions;

pub struct State {
    pub label: u32,
    pub is_final: bool,
    transitions: Transitions,
}

impl State {
    pub fn new(label: u32, is_final: bool, transitions: Transitions) -> State {
        State {
            label,
            is_final,
            transitions,
        }
    }

    pub fn has_same_label(&self, other: &State) -> bool {
        self.label.eq(&other.label)
    }

    pub fn equal(&self, other: &State) -> bool {
        true && self.label.eq(&other.label)
            && self.is_final.eq(&other.is_final)
            && self.transitions.eq(&other.transitions)
    }

    pub fn has_label(&self, label: u32) -> bool {
        self.label.eq(&label)
    }

    pub fn get_transition_for(&self, symbol: char) -> Option<u32> {
        self.transitions.get_transition_for(symbol)
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {}",
            self.label,
            if self.is_final { 1 } else { 0 },
            self.transitions.count(),
            self.transitions.to_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_state() -> State {
        State::new(
            0,
            false,
            Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 3)]),
        )
    }

    #[test]
    fn it_accept_comparision_if_they_are_excactly_the_same() {
        let state1 = get_sample_state();
        let state2 = get_sample_state();

        assert_eq!(state1.equal(&state2), true)
    }

    #[test]
    fn it_rejects_comparision_if_labels_are_not_the_same() {
        let state1 = get_sample_state();
        let state2 = State::new(
            1,
            false,
            Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 3)]),
        );

        assert_eq!(state1.equal(&state2), false)
    }

    #[test]
    fn it_rejects_comparision_if_final_state_differ() {
        let state1 = get_sample_state();
        let state2 = State::new(
            0,
            true,
            Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 3)]),
        );

        assert_eq!(state1.equal(&state2), false)
    }

    #[test]
    fn it_rejects_comparision_if_transitions_differ() {
        let state1 = get_sample_state();
        let state2 = State::new(
            0,
            false,
            Transitions::new(vec![Transition::new('b', 1), Transition::new('c', 3)]),
        );

        assert_eq!(state1.equal(&state2), false)
    }

    #[test]
    fn it_can_be_converted_to_string() {
        let state1 = get_sample_state();
        let expected_string = String::from("0 0 2 a 1 b 3");

        assert_eq!(state1.to_string(), expected_string)
    }
}
