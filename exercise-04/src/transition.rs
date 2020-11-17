#[derive(Debug)]
pub struct Transition {
    symbol: char,
    pub destination_state: u32,
}

impl Transition {
    pub fn new(symbol: char, destination_state: u32) -> Transition {
        Transition {
            symbol,
            destination_state,
        }
    }

    pub fn has_symbol(&self, symbol: char) -> bool {
        self.symbol.eq(&symbol)
    }

    pub fn has_same_symbol(&self, other: &Transition) -> bool {
        self.symbol.eq(&other.symbol)
    }

    pub fn eq(&self, other: &Transition) -> bool {
        self.has_same_symbol(&other) && self.destination_state.eq(&other.destination_state)
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.symbol, self.destination_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_accept_comparison_if_they_are_exactly_the_same() {
        let transition1 = Transition::new('a', 1);
        let transition2 = Transition::new('a', 1);

        assert_eq!(transition1.eq(&transition2), true)
    }

    #[test]
    fn it_rejects_the_comparison_if_symbols_are_not_the_same() {
        let transition1 = Transition::new('a', 1);
        let transition2 = Transition::new('b', 1);

        assert_eq!(transition1.eq(&transition2), false)
    }

    #[test]
    fn it_rejects_the_comparison_if_destination_state_is_not_the_same() {
        let transition1 = Transition::new('a', 1);
        let transition2 = Transition::new('a', 2);

        assert_eq!(transition1.eq(&transition2), false)
    }

    #[test]
    fn it_can_be_represented_as_string() {
        let transition = Transition::new('a', 1);

        assert_eq!(transition.to_string(), "a 1")
    }
}
