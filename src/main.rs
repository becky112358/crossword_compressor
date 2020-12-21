
mod combinations;
use crate::combinations::get_combinations;

const WORDS: [&str; 10] = [
    "hereby",
    "presented",
    "exist",
    "selected",
    "words",
    "intended",
    "for",
    "initial",
    "program",
    "testing",
];

fn main() {
    let combinations = get_combinations(5);
}

