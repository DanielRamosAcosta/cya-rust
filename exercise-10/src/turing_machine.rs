use crate::states::States;
use crate::tape::Tape;
use crate::movement::Movement;
use crate::state::State;

pub struct TuringMachine {
    initial_state: u32,
    states: States
}

pub struct Status {
    pub state_id: u32,
    pub tapes: Vec<Tape>,
}

impl TuringMachine {
    pub fn new(initial_state: u32, states: States) -> TuringMachine {
        TuringMachine { initial_state, states }
    }

    pub fn execute(&self, input: &String) -> Result<Vec<Tape>, Vec<Tape>> {
        let tape = Tape::from_string_padded_with_blanks(input);

        let mut status = Status {
            state_id: self.initial_state,
            tapes: vec![tape]
        };

        loop {
            let state = self.states.state_with_id(status.state_id).unwrap();

            if state.is_final_state() {
                break
            }

            let tape = status.tapes.last().unwrap();
            let option = state.find_transition_for(tape.read());

            if option.is_none() {
                return Err(status.tapes)
            }

            let transition = option.unwrap();

            let tapes = vec![
                tape.write(transition.write),
                tape.write(transition.write).mov(transition.movement)
            ];

            status = Status {
                state_id: transition.destination_state,
                tapes: status.tapes
                    .clone()
                    .iter().chain(tapes.iter()).map(|t| t.to_owned())
                    .collect()
            }
        }

        return Ok(status.tapes)
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
            State::new(4, true, Transitions::new(vec![])),
        ]))
    }

    #[test]
    fn it_accepts_the_readme_turing_machine() {
        let tm = get_sample_turing_machine();

        let tapes = tm.execute(&String::from("01"));
        let last = tapes.unwrap().last().unwrap();

        assert_eq!(last.rail_to_string(), "$XY$");
        assert_eq!(last.head_to_string(), "  ^ ");
    }
}
