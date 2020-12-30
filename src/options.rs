use std::collections::HashMap;

use crate::crossword::{CrossData, Crossword, Direction};
use crate::letter_map::WordAndLetter;

#[derive(PartialEq)]
enum Comparison {
    First,
    Better,
    AsGood,
    Worse,
    SeedDuplicate,
}

pub fn compare_options<'a>(
    letter_map: &HashMap<char, Vec<WordAndLetter>>,
    crossword: &mut Crossword<'a>,
    best_crosswords: &mut Vec<Crossword<'a>>) {

    for (letter, position, direction) in crossword.crossable_letters() {
        if let Some(crossable_words) = letter_map.get(&letter) {
            for word_and_letter in crossable_words {
                if insert_word(&position, direction, &word_and_letter, crossword) {

                    let crossword_status = compare_crosswords(crossword, best_crosswords);

                    if crossword_status == Comparison::Worse || crossword_status == Comparison::SeedDuplicate {
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

fn insert_word(position: &[i32; 2], direction: Direction, word_l: &WordAndLetter, crossword: &mut Crossword) -> bool {

    let word_index = word_l.word_index;

    let mut insertable = crossword.words[word_index].cross == None;

    insertable = insertable && check_insertable(position, direction, word_l, crossword);

    if insertable {
        let cross_data = CrossData {
            position: get_start_position(position, direction, word_l),
            direction: direction,
            order: crossword.get_next_order(),
        };
        crossword.words[word_index].cross = Some(cross_data);
    }

    return insertable;
}

fn check_insertable(position: &[i32; 2], direction: Direction, word_l: &WordAndLetter, crossword: &Crossword) -> bool {
    let mut insertable = true;

    let (new_start, new_end, new_row) = get_new_start_end_row(position, direction, word_l);

    for word in &crossword.words {
        if let Some(cross_data) = &word.cross {

            let (old_start, old_end, old_row) = get_old_start_end_row(word.word, &cross_data);

            if cross_data.direction == direction {
                insertable = insertable && check_same_direction(new_start, new_end, new_row,
                                                                old_start, old_end, old_row, direction, crossword);
            } else {
                insertable = insertable && check_different_direction(new_start, new_end, new_row, word_l.word,
                                                                     old_start, old_end, old_row, word.word);
            }
        }

        if !insertable {
            break;
        }
    }

    return insertable;
}

fn get_new_start_end_row(middle: &[i32; 2], direction: Direction, word_and_letter: &WordAndLetter) -> (i32, i32, i32) {

    let index = direction.index();
    let other = direction.change().index();

    let start = middle[index] - word_and_letter.letter_index as i32;
    let end = middle[index] + word_and_letter.n_letters_after as i32;
    let row = middle[other];

    return (start, end, row);
}

fn get_old_start_end_row(word: &str, cross_data: &CrossData) -> (i32, i32, i32) {
    let index = cross_data.direction.index();
    let other = cross_data.direction.change().index();

    let start = cross_data.position[index];
    let end = cross_data.position[index] + word.len() as i32 - 1;
    let row = cross_data.position[other];

    return (start, end, row);
}

fn check_same_direction(start0: i32, end0: i32, row0: i32,
                        start1: i32, end1: i32, row1: i32,
                        direction: Direction, crossword: &Crossword) -> bool {

    let ok;

    if row0 < row1 - 1 || row1 < row0 - 1 {
        ok = true;
    } else if row0 == row1 - 1 || row1 == row0 - 1 {
        if end0 < start1 || end1 < start0 {
            ok = true;
        } else if end0 == start1 {
            ok = check_intersecting_word(row0, row1, end0, direction.change(), crossword);
        } else if end1 == start0 {
            ok = check_intersecting_word(row0, row1, end1, direction.change(), crossword);
        } else {
            ok = false;
        }
    } else if end0 < start1 - 1 || end1 < start0 - 1 {
        ok = true;
    } else {
        ok = false;
    }

    return ok;
}

fn check_intersecting_word(point0: i32, point1: i32, row: i32, direction: Direction, crossword: &Crossword) -> bool {
    let mut exists = false;

    for word in &crossword.words {
        if let Some(cross_data) = &word.cross {
            if cross_data.direction == direction {
                let index = cross_data.direction.index();
                let other = cross_data.direction.change().index();

                if cross_data.position[other] == row
                && cross_data.position[index] <= point0.min(point1)
                && cross_data.position[index] + word.word.len() as i32 - 1 >= point0.max(point1) {
                    exists = true;
                    break;
                }
            }
        }
    }

    return exists;
}

fn check_different_direction(start0: i32, end0: i32, row1: i32, word0: &str,
                             start1: i32, end1: i32, row0: i32, word1: &str) -> bool {

    let ok;

    if end0 < row0 - 1 || start0 > row0 + 1 || end1 < row1 - 1 || start1 > row1 + 1 {
        ok = true;
    } else if (end0 == row0 - 1 && start1 <= row1 && row1 <= end1)
           || (start0 == row0 + 1 && start1 <= row1 && row1 <= end1)
           || (end1 == row1 - 1 && start0 <= row0 && row0 <= end0)
           || (start1 == row1 + 1 && start0 <= row0 && row0 <= end0) {
        ok = false;
    } else {
        let letter0 = get_nth_letter(word0, row0 - start0);
        let letter1 = get_nth_letter(word1, row1 - start1);
        ok = letter0 == letter1;
    }

    return ok;
}

fn get_nth_letter(word: &str, index: i32) -> char {
    let mut letter = ' ';
    for (i, l) in word.chars().enumerate() {
        if i as i32 == index {
            letter = l;
            break;
        }
    }
    return letter;
}

fn get_start_position(position: &[i32; 2], direction: Direction, word_and_letter: &WordAndLetter) -> [i32; 2] {

    let mut position_start = position.clone();
    let index = direction.index();
    position_start[index] -= word_and_letter.letter_index as i32;

    return position_start;
}

fn compare_crosswords(crossword: &Crossword, best_crosswords: &Vec<Crossword>) -> Comparison {
    let comparison;

    if best_crosswords.len() == 0 {
        comparison = Comparison::First;
    } else {
        let (current_min, current_max) = best_crosswords[0].get_min_max();
        let (new_min, new_max) = crossword.get_min_max();

        if (new_max > current_max) || (new_max == current_max && new_min > current_min) {
            comparison = Comparison::Worse;
        } else if is_duplicate(crossword, best_crosswords) {
            comparison = Comparison::SeedDuplicate;
        } else if (new_max == current_max) && (new_min == current_min) {
            comparison = Comparison::AsGood;
        } else {
            comparison = Comparison::Better;
        }
    }

    return comparison;
}

fn is_duplicate(crossword: &Crossword, best_crosswords: &Vec<Crossword>) -> bool {
    let mut duplicate = false;

    for good_crossword in best_crosswords {
        let mut subset = true;

        let mut order = vec![false; crossword.words.len()];
        let mut good_order = vec![false; crossword.words.len()];

        for word_index in 0..crossword.words.len() {
            if let Some(cross_data) = &crossword.words[word_index].cross {
                if let Some(good_cross_data) = &good_crossword.words[word_index].cross {

                    if cross_data.position != good_cross_data.position
                    || cross_data.direction != good_cross_data.direction {
                        subset = false;
                        break;
                    }

                    order[cross_data.order] = true;
                    good_order[good_cross_data.order] = true;
                }
            }
        }

        if subset {
            for order_index in 0..order.len() {
                if order[order_index] != good_order[order_index] {
                    subset = false;
                    break;
                }
            }
        }

        if subset {
            duplicate = true;
            break;
        }
    }

    return duplicate;
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
        Comparison::First | Comparison::Better | Comparison::AsGood => best_crosswords.push(crossword.clone()),
        Comparison::Worse | Comparison::SeedDuplicate => (),
    }
}

fn remove_word(word_and_letter: &WordAndLetter, crossword: &mut Crossword) {
    let word_index = word_and_letter.word_index;
    crossword.words[word_index].cross = None;
}


#[cfg(test)]
#[path = "./tests_options.rs"]
mod tests_options;


