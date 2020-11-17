mod movement;
mod state;
mod states;
mod tape;
mod transition;
mod transitions;
mod turing_machine;

use crate::movement::Movement;
use crate::state::State;
use crate::states::States;
use crate::tape::Tape;
use crate::transition::Transition;
use crate::transitions::Transitions;
use crate::turing_machine::TuringMachine;
use std::io;
use std::{thread, time};

fn main() {
    let tm = get_sample_dfa();

    let mut input = String::new();
    io::stdin().read_line(&mut input);

    let result = tm.execute(&input.trim().to_string());

    match result {
        Ok(tapes) => {
            pretty_print_execution(tapes);
            println!("Accepted");
        }
        Err(tapes) => {
            pretty_print_execution(tapes);
            println!("Not accepted")
        }
    }
}

fn pretty_print_execution(tapes: Vec<Tape>) {
    for tape in tapes {
        print!("{}[2J", 27 as char);
        println!("{}", tape.to_string());
        let ten_millis = time::Duration::from_millis(100);

        thread::sleep(ten_millis);
    }
}

pub fn get_sample_dfa() -> TuringMachine {
    TuringMachine::new(
        0,
        States::new(vec![
            State::new(
                0,
                false,
                Transitions::new(vec![
                    Transition::new('0', 'X', Movement::RIGHT, 1),
                    Transition::new('Y', 'Y', Movement::RIGHT, 3),
                ]),
            ),
            State::new(
                1,
                false,
                Transitions::new(vec![
                    Transition::new('0', '0', Movement::RIGHT, 1),
                    Transition::new('Y', 'Y', Movement::RIGHT, 1),
                    Transition::new('1', 'Y', Movement::LEFT, 2),
                ]),
            ),
            State::new(
                2,
                false,
                Transitions::new(vec![
                    Transition::new('0', '0', Movement::LEFT, 2),
                    Transition::new('Y', 'Y', Movement::LEFT, 2),
                    Transition::new('X', 'X', Movement::RIGHT, 0),
                ]),
            ),
            State::new(
                3,
                false,
                Transitions::new(vec![
                    Transition::new('Y', 'Y', Movement::RIGHT, 3),
                    Transition::new('$', '$', Movement::LEFT, 4),
                ]),
            ),
            State::new(4, true, Transitions::new(vec![])),
        ]),
    )
}
