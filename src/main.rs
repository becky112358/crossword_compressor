mod crossword;
mod letters;
mod options;
use crate::options::options_compare;
mod output;

fn main() {
    let words_input = vec!["hereby", "exist", "words", "for", "sample", "output"];

    let words = letters::to_lowercase(&words_input);

    let letter_map = letters::get_map(&words);

    let mut crossword = crossword::initialise(&words);

    let mut best_options = vec![];
    options_compare(&letter_map, &mut crossword, &mut best_options);

    output::clear_message(&format!("{} excellent options", best_options.len()));
    for crossword in &best_options {
        crossword.print();
    }
}
