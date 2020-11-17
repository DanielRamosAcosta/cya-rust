use crate::transition_log::TransitionLog;

#[derive(Debug, Clone)]
pub struct TransitionsLog {
    pub transitions: Vec<TransitionLog>,
}

impl TransitionsLog {
    pub fn create() -> TransitionsLog {
        let transitions: Vec<TransitionLog> = Vec::new();

        TransitionsLog { transitions }
    }

    pub fn new(transitions: Vec<TransitionLog>) -> TransitionsLog {
        TransitionsLog { transitions }
    }

    pub fn append(&self, transition: TransitionLog) -> TransitionsLog {
        let aux = vec![transition];
        let transitions = self
            .transitions
            .iter()
            .copied()
            .chain(aux.iter().copied())
            .collect();
        return TransitionsLog::new(transitions);
    }

    pub fn get_last_transition_or_(&self, initial_state: u32) -> u32 {
        self.transitions
            .last()
            .map(|transition| transition.get_next_state())
            .unwrap_or(initial_state)
    }

    pub fn has_consumed_all_the_input(&self, input_length: usize) -> bool {
        self.transitions.len().eq(&input_length)
    }

    pub fn count(&self) -> usize {
        self.transitions.len()
    }

    pub fn to_string(&self) -> String {
        TransitionsLog::get_header()
            + &self
                .transitions
                .iter()
                .map(|&transition| transition.to_string())
                .collect::<Vec<_>>()
                .join("\n")
    }

    fn get_header() -> String {
        String::from("Current State  Input  Next State\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_new_logs() {
        let transition = TransitionLog::new(0, 'a', 0);
        let transitions = TransitionsLog::create();
        let count_transitions = transitions.append(transition).count();

        assert_eq!(count_transitions, 1)
    }
}
