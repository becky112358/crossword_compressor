
mod crossword;
use crate::crossword::initialise_crossword;
mod letter_map;
use crate::letter_map::get_letter_map;
mod options;
use crate::options::compare_options;

fn main() {
    let words = vec![
        "hereby",
        "exist",
        "words",
        "for",
        "testing",
    ];

    let letter_map = get_letter_map(&words);

    let mut crossword = initialise_crossword(&words, "beginning");

    let mut words_in_crossword = vec![false; words.len()];
    let mut best_options = vec![];
    compare_options(&letter_map, &mut words_in_crossword, &mut crossword, &mut best_options);

    for crossword in &best_options {
        crossword.print();
        println!();
    }
    println!("{} options", best_options.len());
}

