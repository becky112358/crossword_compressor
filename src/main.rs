
mod crossword;
use crate::crossword::initialise_crossword;
mod letter_map;
use crate::letter_map::get_letter_map;
mod options;
use crate::options::compare_options;

fn main() {
    let words = vec![
        "beginning",
        "hereby",
        "exist",
        "words",
        "for",
        "testing",
    ];

    let letter_map = get_letter_map(&words);

    let mut crossword = initialise_crossword(&words);

    let mut best_options = vec![];
    compare_options(&letter_map, &mut crossword, &mut best_options);

    for crossword in &best_options {
        crossword.print();
    }
    println!("{} options", best_options.len());
}

