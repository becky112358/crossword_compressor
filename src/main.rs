
mod crossword;
use crate::crossword::crossword_initialise;
mod letters;
use crate::letters::{letters_to_lowercase, letters_get_map};
mod options;
use crate::options::options_compare;
mod output;
use crate::output::output_clear_message;

fn main() {
    let words_input = vec![
        "small",
        "collection",
        "of",
        "words",
    ];

    let words = letters_to_lowercase(&words_input);

    let letter_map = letters_get_map(&words);

    let mut crossword = crossword_initialise(&words);

    let mut best_options = vec![];
    options_compare(&letter_map, &mut crossword, &mut best_options);

    output_clear_message(&format!("{} excellent options", best_options.len()));
    for crossword in &best_options {
        crossword.print();
    }
}

