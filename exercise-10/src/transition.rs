use crate::movement::Movement;

#[derive(Debug)]
pub struct Transition {
    pub read: char,
    pub write: char,
    pub movement: Movement,
    pub destination_state: u32,
}

impl Transition {
    pub fn new(
        read: char,
        write: char,
        movement: Movement,
        destination_state: u32,
    ) -> Transition {
        Transition {
            read,
            write,
            movement,
            destination_state,
        }
    }
}