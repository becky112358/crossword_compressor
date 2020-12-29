use std::collections::HashMap;

use crate::crossword::{CrossData, Crossword, Direction, WordCross, X, Y};
use crate::letter_map::WordAndLetter;

#[derive(PartialEq)]
enum Comparison {
    First,
    Better,
    AsGood,
    Worse,
// todo Add seed subset
}

pub fn compare_options<'a>(
    letter_map: &HashMap<char, Vec<WordAndLetter>>,
    crossword: &mut Crossword<'a>,
    best_crosswords: &mut Vec<Crossword<'a>>) {

    for (letter, position, direction) in crossword.crossable_letters() {
        if let Some(crossable_words) = letter_map.get(&letter) {
            for word_and_letter in crossable_words {
                if insert_word(&position, &direction, &word_and_letter, crossword) {
                    let crossword_status = compare_crosswords(crossword, best_crosswords);

                    if crossword_status == Comparison::Worse {
                    } else if crossword.all_words_crossed() {
                        add_crossword(crossword_status, crossword, best_crosswords);
                    } else {
                        compare_options(letter_map, crossword, best_crosswords);
                    }
                    remove_word(&word_and_letter, crossword);
                }
            }
        }
    }
}

fn insert_word(
    position: &[i32; 2],
    direction: &Direction,
    word_and_letter: &WordAndLetter,
    crossword: &mut Crossword) -> bool {

    let word_index = word_and_letter.word_index;

    let mut insertable = true;

    match crossword.words[word_index].cross {
        None => (),
        _ => insertable = false,
    }

    let (position_start, position_end) = get_position_start_end(position, direction, word_and_letter);

    insertable = insertable && check_no_same_direction_overlaps(&position_start, &position_end, direction, crossword);

    insertable = insertable && check_other_direction_overlaps();

    if insertable {
        // todo Bug: position needs to be at start of word, not at letter crossing
        let cross_data = CrossData {
            position: position.clone(),
            direction: direction.clone(),
            order: crossword.get_next_order(),
        };
        crossword.words[word_index].cross = Some(cross_data);
    }

    return insertable;
}

fn get_position_start_end(
    position: &[i32; 2],
    direction: &Direction,
    word_and_letter: &WordAndLetter) -> ([i32; 2], [i32; 2]) {

    let index;
    match *direction {
        Direction::Across => index = X,
        Direction::Down => index = Y,
    }

    let mut position_start = position.clone();
    let mut position_end = position.clone();

    position_start[index] -= word_and_letter.n_letters_before as i32;
    position_end[index] += word_and_letter.n_letters_after as i32;

    return (position_start, position_end);
}

fn check_no_same_direction_overlaps(
    position_start: &[i32; 2],
    position_end: &[i32; 2],
    direction: &Direction,
    crossword: &Crossword) -> bool {

    let mut clear = true;

    let z = match direction { Direction::Across => X, Direction::Down => Y, };

    let start_z = position_start[z];
    let end_z = position_end[z];
    let row = position_start[(z + 1) % 2];

    clear = clear && check_row_clear(z, row, start_z-1, end_z+1, crossword);

    clear = clear && check_row_clear(z, row+1, start_z, end_z, crossword);

    clear = clear && check_row_clear(z, row-1, start_z, end_z, crossword);

    return clear;
}

fn check_row_clear(z: usize, row: i32, start: i32, end: i32, crossword: &Crossword) -> bool {
    let mut clear = true;

    let not_z = (z + 1) % 2;

    for word in &crossword.words {
        if let Some(cross_data) = &word.cross {
            if cross_data.position[not_z] == row {
                if cross_data.position[z] <= start && cross_data.position[z] + word.word.len() as i32 >= start {
                    clear = false;
                    break;
                } else if cross_data.position[z] >= start && cross_data.position[z] <= end {
                    clear = false;
                    break;
                }
            }
        }
    }

    return clear;
}

// todo write function
fn check_other_direction_overlaps() -> bool {

    return true;
}

fn compare_crosswords(crossword: &Crossword, best_crosswords: &Vec<Crossword>) -> Comparison {
    let comparison;

    if best_crosswords.len() == 0 {
        comparison = Comparison::First;
    } else {
        let (current_min, current_max) = best_crosswords[0].get_min_max();
        let (new_min, new_max) = crossword.get_min_max();

        if (new_max < current_max) || (new_max == current_max && new_min < current_min) {
            comparison = Comparison::Better;
        } else if (new_max == current_max) && (new_min == current_min) {
            comparison = Comparison::AsGood;
        } else {
            comparison = Comparison::Worse;
        }
    }

    return comparison;
}

fn add_crossword<'a>(comparison: Comparison, crossword: &Crossword<'a>, best_crosswords: &mut Vec<Crossword<'a>>) {
    match comparison {
        Comparison::Better => {
            while best_crosswords.len() > 0 {
                best_crosswords.pop();
            }
        }
        _ => (),
    }

    match comparison {
        Comparison::First | Comparison::Better | Comparison::AsGood => {

            // todo yuck
            let mut word_cross_vec = Vec::with_capacity(crossword.words.len());

            for index in 0..crossword.words.len() {
                let word_cross = WordCross {
                    word: crossword.words[index].word,
                    cross: crossword.words[index].cross.clone(),
                };
                word_cross_vec.push(word_cross);
            }
            let good_crossword = Crossword {
                words: word_cross_vec,
            };

            best_crosswords.push(good_crossword);
        }
        Comparison::Worse => {}
    }
}

fn remove_word(word_and_letter: &WordAndLetter, crossword: &mut Crossword) {
    let word_index = word_and_letter.word_index;
    crossword.words[word_index].cross = None;
}

