mod turing_machine;
mod states;
mod transition;
mod transitions;
mod state;
mod movement;
mod tape;

use std::io;

fn main() {
    let mut input = String::new();
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    let result = io::stdin()
        .read_line(&mut input)
        .map(|size| {
            size * 3
        })
        .unwrap();

    let first_char = input
        .chars()
        .next()
        .map(|char| String::from(char))
        .and_then(|char| char.parse::<i32>().ok())
        .map(|digit| numbers.iter().find(|x| x.eq(&&digit)))
        .map(|digit| digit.is_some());

    println!("{}", first_char.unwrap_or(false));


    println!("Hello world! {}", result);
}
