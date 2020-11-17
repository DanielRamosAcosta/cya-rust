use crate::dfa::Dfa;
use crate::state::State;
use crate::transition::Transition;
use crate::transitions::Transitions;

pub fn get_initial_state(string: &String) -> Option<u32> {
    string
        .chars()
        .skip(2)
        .take(1)
        .collect::<String>()
        .parse::<u32>()
        .ok()
}

pub fn get_body(string: &String) -> String {
    string
        .chars()
        .skip(4)
        .take(string.len() - 4)
        .collect::<String>()
}

fn parse_transition(el: Vec<String>) -> Transition {
    let symbol = el.get(0).unwrap();
    let dest_state = el.get(1).unwrap().parse::<u32>().unwrap();

    let symbol1 = symbol.chars().next().unwrap();
    return Transition::new(symbol1, dest_state);
}

pub fn parse_state(string: &String) -> State {
    let elements = string
        .split(' ')
        .map(|str| String::from(str))
        .collect::<Vec<String>>();

    let label = elements
        .get(0)
        .map(|x| x.parse::<u32>())
        .and_then(|x| x.ok());

    let is_final = elements
        .get(1)
        .map(|x| x.parse::<u32>())
        .and_then(|x| x.ok())
        .map(|x| x != 0);

    let transitions_str = &elements[3..];

    let transitions: Vec<Transition> = transitions_str
        .chunks(2)
        .map(|x| x.to_vec())
        .map(parse_transition)
        .collect();

    State::new(
        label.unwrap(),
        is_final.unwrap(),
        Transitions::new(transitions),
    )
}

pub fn parse_dfa(string: String) -> Option<Dfa> {
    let dfa_body = get_body(&string);

    let dfa_initial_state = get_initial_state(&string);

    let states = dfa_body
        .split('\n')
        .map(|str| String::from(str))
        .map(|str| parse_state(&str))
        .collect::<Vec<State>>();

    let dfa = Dfa::new(dfa_initial_state.unwrap(), states);

    return Some(dfa);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;
    use crate::transition::Transition;
    use crate::transitions::Transitions;

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
        let dfa1 = get_sample_dfa();

        let dfa2 = parse_dfa(String::from(
            "4
0
0 0 2 a 1 b 3
1 1 2 a 1 b 2
2 1 2 a 1 b 3
3 0 2 a 3 b 3",
        ))
        .unwrap();

        assert_eq!(dfa1.to_string(), dfa2.to_string())
    }
}
