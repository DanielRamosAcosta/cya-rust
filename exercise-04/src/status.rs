use crate::transitions_log::TransitionsLog;

#[derive(Debug, Clone)]
pub struct Status {
    input: String,
    transitions_log: TransitionsLog,
    is_accepted: bool,
}

impl Status {
    pub fn new(input: String, transitions: TransitionsLog, is_accepted: bool) -> Status {
        Status {
            input,
            transitions_log: transitions,
            is_accepted,
        }
    }

    pub fn to_string(&self) -> String {
        let input = String::from("Input string: <input>").replace("<input>", &self.input);
        let transitions = self.transitions_log.to_string();
        let accepted = if self.is_accepted {
            String::from("Input is ACCEPTED")
        } else {
            String::from("Input is REJECTED")
        };

        let result = input + "\n" + &transitions + "\n" + &accepted;

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transition_log::TransitionLog;

    #[test]
    fn it_shows_the_input_string() {
        let result = Status::new(
            String::from("abaab"),
            TransitionsLog::new(Vec::new()),
            false,
        )
        .to_string();

        assert!(result.contains("abaab"))
    }

    #[test]
    fn it_shows_the_transaction_string() {
        let transitions = vec![TransitionLog::new(0, 'a', 1), TransitionLog::new(1, 'b', 2)];
        let result = Status::new(
            String::from("abaab"),
            TransitionsLog::new(transitions),
            false,
        )
        .to_string();

        assert!(result.contains("0              a      1"));
        assert!(result.contains("1              b      2"));
    }

    #[test]
    fn it_shows_the_if_its_accepted() {
        let result =
            Status::new(String::from("abaab"), TransitionsLog::new(Vec::new()), true).to_string();

        assert!(result.contains("Input is ACCEPTED"))
    }

    #[test]
    fn it_shows_the_if_its_rejected() {
        let result = Status::new(
            String::from("abaab"),
            TransitionsLog::new(Vec::new()),
            false,
        )
        .to_string();

        assert!(result.contains("Input is REJECTED"))
    }
}
