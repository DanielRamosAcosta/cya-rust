use crate::transitions::Transitions;
use crate::movement::Movement;

#[derive(Clone)]
pub struct Tape {
    rail: Vec<char>,
    head: usize,
}

impl Tape {
    pub fn from_string(string: &str) -> Tape {
        Tape::from_string_head_at(string, 0)
    }

    pub fn from_string_head_at(string: &str, head: usize) -> Tape {
        let chars = string.chars().collect::<Vec<char>>();

        Tape::new(chars, head)
    }

    pub fn new(rail: Vec<char>, head: usize) -> Tape {
        Tape { rail, head }
    }

    pub fn mov(&self, movement: Movement) -> Tape {
        match movement {
            Movement::LEFT => Tape::new(self.rail.clone(), self.head - 1),
            Movement::RIGHT => Tape::new(self.rail.clone(), self.head + 1),
            Movement::STOP => self.clone(),
        }
    }

    pub fn write (&self, char: char) -> Tape {
        let mut other = self.rail.clone();

        other[self.head] = char;

        Tape::new(other, self.head)
    }

    pub fn to_string (&self) -> String {
        format!(
            "...{}...\n...{}...",
            self.rail_to_string(),
            self.head_to_string()
        )
    }

    pub fn head_to_string(&self) -> String {
        self.rail_to_string()
            .chars()
            .into_iter()
            .enumerate()
            .map(|(i, x)| i)
            .map(|x| self.index_to_symbol(&x))
            .collect::<Vec<String>>()
            .join("")
    }

    fn index_to_symbol(&self, i: &usize) -> String {
        if self.head.eq(i) {
            String::from("^")
        } else {
            String::from(" ")
        }
    }

    pub fn rail_to_string(&self) -> String {
        self.rail
            .iter()
            .map(|x| String::from(x.clone()))
            .collect::<Vec<String>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::movement::Movement;

    #[test]
    fn it_can_be_represented_as_string() {
        let tape = Tape::from_string("000111");

        let result = tape.to_string();

        assert_eq!(tape.rail_to_string(), "000111");
        assert_eq!(tape.head_to_string(), "^     ");
        assert_eq!(result, "...000111...
...^     ...");
    }

    #[test]
    fn head_can_be_moved_right() {
        let tape = Tape::from_string("000111");

        let result = tape.mov(Movement::RIGHT);

        assert_eq!(result.rail_to_string(), "000111");
        assert_eq!(result.head_to_string(), " ^    ");
    }

    #[test]
    fn head_can_be_moved_left() {
        let tape = Tape::from_string_head_at("000111", 2);

        let result = tape.mov(Movement::LEFT);

        assert_eq!(result.rail_to_string(), "000111");
        assert_eq!(result.head_to_string(), " ^    ");
    }

    #[test]
    fn head_can_be_moved_a_lot() {
        let tape = Tape::from_string("000111");

        Tape::mov(&tape, Movement::RIGHT);
        tape.mov(Movement::RIGHT);

        let result = tape
            .mov(Movement::RIGHT)
            .mov(Movement::RIGHT)
            .mov(Movement::STOP)
            .mov(Movement::LEFT)
            .mov(Movement::LEFT)
            .mov(Movement::STOP);

        assert_eq!(result.rail_to_string(), "000111");
        assert_eq!(result.head_to_string(), "^     ");
    }

    #[test]
    fn write_works_at_start() {
        let tape = Tape::from_string("000111");

        let result = tape.write('X');

        assert_eq!(result.rail_to_string(), "X00111");
    }

    #[test]
    fn write_works_with_moved_head() {
        let tape = Tape::from_string("000111");

        let result = tape.mov(Movement::RIGHT).write('X');

        assert_eq!(result.rail_to_string(), "0X0111");
    }
}
