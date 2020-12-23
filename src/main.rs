
mod combinations;
use crate::combinations::get_combinations;
mod common;
mod crossword;
use crate::crossword::initialise_crossword;
mod letter_map;
use crate::letter_map::{create_word_usages, get_letter_map};
mod options;
use crate::options::compare_options;

fn main() {
    let words = vec![
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

    let combinations = get_combinations(5);
    let mut word_usages = create_word_usages(&words);
    let mut letter_map = get_letter_map(&mut word_usages);

    let mut crossword = initialise_crossword(&words, "beginning");

//    let best_options = compare_options(&mut letter_map, &mut crossword);

//    for crossword in best_options {
//        crossword.print();
//    }
}

