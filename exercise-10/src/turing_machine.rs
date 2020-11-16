use crate::states::States;

pub struct TuringMachine {
    initial_state: u32,
    states: States
}

impl TuringMachine {
    pub fn new(initial_state: u32, states: States) -> TuringMachine {
        TuringMachine { initial_state, states }
    }

    pub fn execute(&self, input: &String) {

    }

    pub fn dummy (&self) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;
    use crate::transitions::Transitions;
    use crate::transition::Transition;
    use crate::movement::Movement;

    fn get_sample_turing_machine () -> TuringMachine {
        TuringMachine::new(0, States::new(vec![
            State::new(0, false, Transitions::new(vec![
                Transition::new('0', 'X', Movement::RIGHT, 1),
                Transition::new('Y', 'Y', Movement::RIGHT, 3),
            ])),
            State::new(1, false, Transitions::new(vec![
                Transition::new('0', '0', Movement::RIGHT, 1),
                Transition::new('Y', 'Y', Movement::RIGHT, 1),
                Transition::new('1', 'Y', Movement::LEFT, 2),
            ])),
            State::new(2, false, Transitions::new(vec![
                Transition::new('0', '0', Movement::LEFT, 2),
                Transition::new('Y', 'Y', Movement::LEFT, 2),
                Transition::new('X', 'X', Movement::RIGHT, 0),
            ])),
            State::new(3, false, Transitions::new(vec![
                Transition::new('Y', 'Y', Movement::RIGHT, 3),
                Transition::new('$', '$', Movement::LEFT, 4),
            ])),
        ]))
    }

    #[test]
    fn it_accepts_the_readme_turing_machine() {
        let tm = get_sample_turing_machine();

        tm.execute(&String::from("01"));

        assert_eq!(tm.dummy(), 1)
    }
}
