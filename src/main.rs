
mod crossword;
use crate::crossword::{initialise_crossword, print_clear_message};
mod letters;
use crate::letters::{letters_to_lowercase, letters_get_map};
mod options;
use crate::options::compare_options;

fn main() {
    let words_input = vec![
        "beginning",
        "hereby",
        "exist",
        "words",
        "for",
        "testing",
    ];

    let words = letters_to_lowercase(&words_input);

    let letter_map = letters_get_map(&words);

    let mut crossword = initialise_crossword(&words);

    let mut best_options = vec![];
    compare_options(&letter_map, &mut crossword, &mut best_options);

    print_clear_message(&format!("{} excellent options", best_options.len()));
    for crossword in &best_options {
        crossword.print();
    }
}

