#[derive(Debug, Copy, Clone)]
pub struct TransitionLog {
    current_state: u32,
    input: char,
    next_state: u32,
}

impl TransitionLog {
    pub fn new(current_state: u32, input: char, next_state: u32) -> TransitionLog {
        return TransitionLog {
            current_state,
            input,
            next_state,
        };
    }

    pub fn get_next_state(&self) -> u32 {
        self.next_state
    }

    pub fn to_string(&self) -> String {
        return format!(
            "{}              {}      {}",
            self.current_state.to_string(),
            self.input.to_string(),
            self.next_state.to_string()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_transforms_to_string() {
        let transition = TransitionLog::new(0, 'a', 0);

        assert_eq!(transition.to_string(), "0              a      0")
    }
}
