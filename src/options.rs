use std::collections::HashMap;

use crate::crossword::{CrossData, Crossword, Direction};
use crate::letters::WordAndLetter;
use crate::output::output_clear_message;

#[derive(Debug, PartialEq)]
enum Comparison {
    First,
    Better,
    AsGood,
    Worse,
    SeedDuplicate,
}

pub fn options_compare<'a>(
    letter_map: &HashMap<char, Vec<WordAndLetter>>,
    crossword: &mut Crossword<'a>,
    best_crosswords: &mut Vec<Crossword<'a>>,
) {
    for (letter, row, mid_point, direction) in crossword.get_crossable_letters() {
        if let Some(crossable_words) = letter_map.get(&letter) {
            for word_and_letter in crossable_words {
                if insert_word(row, mid_point, direction, word_and_letter, crossword) {
                    let crossword_status = compare_crosswords(crossword, best_crosswords);

                    if crossword_status == Comparison::Worse
                        || crossword_status == Comparison::SeedDuplicate
                    {
                    } else if crossword.all_words_crossed() {
                        add_crossword(crossword_status, crossword, best_crosswords);
                    } else {
                        options_compare(letter_map, crossword, best_crosswords);
                    }

                    remove_word(word_and_letter, crossword);
                }
            }
        }
    }
}

fn insert_word(
    row: i32,
    mid_p: i32,
    direction: Direction,
    word_l: &WordAndLetter,
    crossword: &mut Crossword,
) -> bool {
    let word_index = word_l.word_index;

    if crossword.words[word_index].cross.is_some() {
        return false;
    }

    let start_point = mid_p - word_l.letter_index as i32;
    if !check_insertable(row, start_point, direction, word_l, crossword) {
        return false;
    }

    let cross_data = CrossData {
        row,
        start_point,
        direction,
        order: crossword.get_next_order(),
    };
    crossword.words[word_index].cross = Some(cross_data);

    true
}

fn check_insertable(
    new_row: i32,
    new_start: i32,
    direction: Direction,
    word_and_letter: &WordAndLetter,
    crossword: &Crossword,
) -> bool {
    let new_end = get_end_point(new_start, word_and_letter.word);

    for word in &crossword.words {
        if let Some(cross_data) = &word.cross {
            let old_start = cross_data.start_point;
            let old_end = get_end_point(old_start, word.word);
            let old_row = cross_data.row;

            if (cross_data.direction == direction
                && !check_same_direction(
                    new_start, new_end, new_row, old_start, old_end, old_row, direction, crossword,
                ))
                || (cross_data.direction != direction
                    && !check_different_direction(
                        new_start,
                        new_end,
                        new_row,
                        word_and_letter.word,
                        old_start,
                        old_end,
                        old_row,
                        word.word,
                    ))
            {
                return false;
            }
        }
    }

    true
}

fn get_end_point(start: i32, word: &str) -> i32 {
    start + word.len() as i32 - 1
}

fn check_same_direction(
    start0: i32,
    end0: i32,
    row0: i32,
    start1: i32,
    end1: i32,
    row1: i32,
    direction: Direction,
    crossword: &Crossword,
) -> bool {
    if row0 < row1 - 1 || row1 < row0 - 1 {
        true
    } else if row0 == row1 - 1 || row1 == row0 - 1 {
        if end0 < start1 || end1 < start0 {
            true
        } else if end0 == start1 {
            check_connecting_word(row0, row1, end0, direction.change(), crossword)
        } else if end1 == start0 {
            check_connecting_word(row0, row1, end1, direction.change(), crossword)
        } else {
            false
        }
    } else {
        end0 < start1 - 1 || end1 < start0 - 1
    }
}

fn check_connecting_word(
    point0: i32,
    point1: i32,
    row: i32,
    direction: Direction,
    crossword: &Crossword,
) -> bool {
    for word in &crossword.words {
        if let Some(cross_data) = &word.cross {
            if cross_data.direction == direction
                && cross_data.row == row
                && cross_data.start_point <= point0.min(point1)
                && cross_data.start_point + word.word.len() as i32 > point0.max(point1)
            {
                return true;
            }
        }
    }

    false
}

fn check_different_direction(
    start0: i32,
    end0: i32,
    row1: i32,
    word0: &str,
    start1: i32,
    end1: i32,
    row0: i32,
    word1: &str,
) -> bool {
    if end0 < row0 - 1 || start0 > row0 + 1 || end1 < row1 - 1 || start1 > row1 + 1 {
        true
    } else if ((end0 == row0 - 1 || start0 == row0 + 1) && start1 <= row1 && row1 <= end1)
        || ((end1 == row1 - 1 || start1 == row1 + 1) && start0 <= row0 && row0 <= end0)
    {
        false
    } else {
        let letter0 = get_nth_letter(word0, row0 - start0);
        let letter1 = get_nth_letter(word1, row1 - start1);
        letter0 == letter1
    }
}

fn get_nth_letter(word: &str, index: i32) -> char {
    for (i, l) in word.chars().enumerate() {
        if i as i32 == index {
            return l;
        }
    }
    ' '
}

fn compare_crosswords(crossword: &Crossword, best_crosswords: &[Crossword]) -> Comparison {
    if best_crosswords.is_empty() {
        Comparison::First
    } else {
        let (current_min, current_max) = best_crosswords[0].get_min_max();
        let (new_min, new_max) = crossword.get_min_max();

        if (new_max > current_max) || (new_max == current_max && new_min > current_min) {
            Comparison::Worse
        } else if is_duplicate(crossword, best_crosswords) {
            Comparison::SeedDuplicate
        } else if (new_max == current_max) && (new_min == current_min) {
            Comparison::AsGood
        } else {
            Comparison::Better
        }
    }
}

fn is_duplicate(crossword: &Crossword, best_crosswords: &[Crossword]) -> bool {
    let mut duplicate = false;

    for good_crossword in best_crosswords {
        let mut subset = true;

        let mut order = vec![false; crossword.words.len()];
        let mut good_order = vec![false; crossword.words.len()];

        for word_index in 0..crossword.words.len() {
            if let Some(cross_data) = &crossword.words[word_index].cross {
                if let Some(good_cross_data) = &good_crossword.words[word_index].cross {
                    if cross_data.row != good_cross_data.row
                        || cross_data.start_point != good_cross_data.start_point
                        || cross_data.direction != good_cross_data.direction
                    {
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

    duplicate
}

fn add_crossword<'a>(
    comparison: Comparison,
    crossword: &Crossword<'a>,
    best_crosswords: &mut Vec<Crossword<'a>>,
) {
    if comparison == Comparison::Better {
        output_clear_message("Selection improved!");
        while !best_crosswords.is_empty() {
            best_crosswords.pop();
        }
    }

    match comparison {
        Comparison::First | Comparison::Better | Comparison::AsGood => {
            crossword.print();
            best_crosswords.push(crossword.clone());
        }
        Comparison::Worse | Comparison::SeedDuplicate => {}
    }
}

fn remove_word(word_and_letter: &WordAndLetter, crossword: &mut Crossword) {
    let word_index = word_and_letter.word_index;
    crossword.words[word_index].cross = None;
}

#[cfg(test)]
#[path = "./tests_options.rs"]
mod tests_options;
