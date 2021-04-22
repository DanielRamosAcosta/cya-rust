use crate::state::State;
use crate::status::Status;
use crate::transition_log::TransitionLog;
use crate::transitions_log::TransitionsLog;

pub struct Dfa {
    initial_state: u32,
    states: Vec<State>,
}

impl Dfa {
    pub fn new(initial_state: u32, states: Vec<State>) -> Dfa {
        Dfa {
            initial_state,
            states,
        }
    }

    pub fn eq(&self, other: &Dfa) -> bool {
        if self.initial_state != other.initial_state {
            return false;
        }

        if self.states.len() != other.states.len() {
            return false;
        }

        self.states.iter().fold(true, |result, state| {
            result
                && other
                    .states
                    .iter()
                    .find(|s| s.has_same_label(state))
                    .map(|s| s.equal(state))
                    .unwrap_or(false)
        })
    }

    pub fn get_state_with_label(&self, label: u32) -> Option<&State> {
        self.states.iter().find(|state| state.has_label(label))
    }

    pub fn execute(&self, input: String) -> Status {
        let transitions = self.perform_transitions(&input);
        let is_final = self.transitions_ended_in_final_state(&input, &transitions);

        return Status::new(input, transitions, is_final);
    }

    pub fn to_string(&self) -> String {
        let states = self
            .states
            .iter()
            .map(|state| state.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        return format!("{}\n{}\n{}", self.states.len(), self.initial_state, states);
    }

    fn transitions_ended_in_final_state(
        &self,
        input: &String,
        transitions: &TransitionsLog,
    ) -> bool {
        self.get_state_with_label(transitions.get_last_transition_or_(self.initial_state))
            .map(|state| state.is_final)
            .map(|is_final| is_final && transitions.has_consumed_all_the_input(input.len()))
            .unwrap_or(false)
    }

    fn append_transition_for_symbol(
        transitions: TransitionsLog,
        symbol: char,
    ) -> impl Fn(&State) -> Option<TransitionsLog> {
        move |state| {
            state
                .get_transition_for(symbol)
                .map(|next_state_label| TransitionLog::new(state.label, symbol, next_state_label))
                .map(|transition_log| transitions.append(transition_log))
        }
    }

    fn perform_transitions(&self, input: &String) -> TransitionsLog {
        input
            .chars()
            .into_iter()
            .fold(TransitionsLog::create(), |transitions, symbol| {
                self.get_state_with_label(transitions.get_last_transition_or_(self.initial_state))
                    .and_then(Dfa::append_transition_for_symbol(
                        transitions.clone(),
                        symbol,
                    ))
                    .unwrap_or(transitions)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transitions::Transitions;
    use crate::transition::Transition;

    fn get_sample_dfa() -> Dfa {
        Dfa::new(
            0,
            vec![
                State::new(
                    0,
                    false,
                    Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 3)]),
                ),
                State::new(
                    1,
                    true,
                    Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 2)]),
                ),
                State::new(
                    2,
                    true,
                    Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 3)]),
                ),
                State::new(
                    3,
                    false,
                    Transitions::new(vec![Transition::new('a', 3), Transition::new('b', 3)]),
                ),
            ],
        )
    }

    #[test]
    fn it_accepts_the_readme_dfa() {
        let dfa = get_sample_dfa();

        let input = String::from("abaab");

        let result = dfa.execute(input);

        assert_eq!(
            result.to_string(),
            String::from(
                "Input string: abaab
Current State  Input  Next State
0              a      1
1              b      2
2              a      1
1              a      1
1              b      2
Input is ACCEPTED"
            )
        )
    }

    #[test]
    fn it_shows_rejected_states() {
        let dfa = get_sample_dfa();

        let input = String::from("b");

        let result = dfa.execute(input);

        assert_eq!(
            result.to_string(),
            String::from(
                "Input string: b
Current State  Input  Next State
0              b      3
Input is REJECTED"
            )
        )
    }

    #[test]
    fn it_rejects_bad_input_strings() {
        let dfa = get_sample_dfa();

        let input = String::from("acdfg");

        let result = dfa.execute(input);

        assert_eq!(
            result.to_string(),
            String::from(
                "Input string: acdfg
Current State  Input  Next State
0              a      1
Input is REJECTED"
            )
        )
    }

    #[test]
    fn it_is_comparable_to_other_dfas() {
        let dfa1 = get_sample_dfa();
        let dfa2 = get_sample_dfa();

        assert_eq!(dfa1.eq(&dfa2), true)
    }

    #[test]
    fn it_rejects_if_number_of_states_is_not_equal() {
        let dfa1 = get_sample_dfa();
        let dfa2 = Dfa::new(
            0,
            vec![State::new(
                0,
                false,
                Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 3)]),
            )],
        );

        assert_eq!(dfa1.eq(&dfa2), false)
    }

    #[test]
    fn it_rejects_comparision_if_states_labels_are_not_the_same() {
        let dfa1 = get_sample_dfa();
        let dfa2 = Dfa::new(
            0,
            vec![
                State::new(
                    5,
                    false,
                    Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 3)]),
                ),
                State::new(
                    6,
                    false,
                    Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 3)]),
                ),
                State::new(
                    7,
                    false,
                    Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 3)]),
                ),
                State::new(
                    8,
                    false,
                    Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 3)]),
                ),
            ],
        );

        assert_eq!(dfa1.eq(&dfa2), false)
    }

    #[test]
    fn it_accepts_the_comparison_if_they_are_equivalent() {
        let dfa1 = get_sample_dfa();
        let dfa2 = Dfa::new(
            0,
            vec![
                State::new(
                    0,
                    false,
                    Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 3)]),
                ),
                State::new(
                    2,
                    true,
                    Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 3)]),
                ),
                State::new(
                    1,
                    true,
                    Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 2)]),
                ),
                State::new(
                    3,
                    false,
                    Transitions::new(vec![Transition::new('a', 3), Transition::new('b', 3)]),
                ),
            ],
        );

        assert_eq!(dfa1.eq(&dfa2), true)
    }

    #[test]
    fn it_can_be_converted_into_string() {
        let dfa1 = get_sample_dfa();
        let string_representation = String::from(
            "4
0
0 0 2 a 1 b 3
1 1 2 a 1 b 2
2 1 2 a 1 b 3
3 0 2 a 3 b 3",
        );

        assert_eq!(dfa1.to_string(), string_representation)
    }
}
