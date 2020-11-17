use crate::transition::Transition;

pub struct Transitions {
    pub transitions: Vec<Transition>,
}

impl Transitions {
    pub fn new(transitions: Vec<Transition>) -> Transitions {
        Transitions { transitions }
    }

    pub fn eq(&self, other: &Transitions) -> bool {
        self.same_amount_of_transitions(other)
            && self.transitions.iter().fold(true, |result, transition| {
                result
                    && other
                        .transitions
                        .iter()
                        .find(|t| t.has_same_symbol(transition))
                        .map(|t| t.eq(transition))
                        .unwrap_or(false)
            })
    }

    pub fn get_transition_for(&self, symbol: char) -> Option<u32> {
        self.transitions
            .iter()
            .find(|x| x.has_symbol(symbol))
            .map(|x| x.destination_state)
    }

    pub fn count(&self) -> usize {
        self.transitions.len()
    }

    pub fn to_string(&self) -> String {
        self.transitions
            .iter()
            .map(|transition| transition.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn same_amount_of_transitions(&self, other: &Transitions) -> bool {
        self.transitions.len().eq(&other.transitions.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_transitions() -> Transitions {
        Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 2)])
    }

    #[test]
    fn it_accept_comparison_if_they_are_exactly_the_same() {
        let transitions1 = get_sample_transitions();
        let transitions2 = get_sample_transitions();

        assert_eq!(transitions1.eq(&transitions2), true)
    }

    #[test]
    fn it_accept_comparison_if_the_order_is_not_the_same() {
        let transitions1 = Transitions::new(vec![Transition::new('b', 2), Transition::new('a', 1)]);
        let transitions2 = Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 2)]);

        assert_eq!(transitions1.eq(&transitions2), true)
    }

    #[test]
    fn it_rejects_the_comparison_if_the_transitions_differ() {
        let transitions1 = get_sample_transitions();
        let transitions2 = Transitions::new(vec![
            Transition::new('a', 1),
            Transition::new('b', 2),
            Transition::new('b', 2),
        ]);

        assert_eq!(transitions1.eq(&transitions2), false)
    }

    #[test]
    fn it_rejects_the_comparison_if_the_amount_of_transitions_differ() {
        let transitions1 = get_sample_transitions();
        let transitions2 = Transitions::new(vec![Transition::new('a', 1), Transition::new('b', 1)]);

        assert_eq!(transitions1.eq(&transitions2), false)
    }
}
