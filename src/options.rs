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
                if insert_word(&position, &direction, &word_and_letter, crossword) {
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

fn insert_word(
    position: &[i32; 2],
    direction: &Direction,
    word_and_letter: &WordAndLetter,
    crossword: &mut Crossword) -> bool {

    let word_index = word_and_letter.word_index;

    let mut insertable = crossword.words[word_index].cross == None;

    let (position_start, position_end) = get_position_start_end(position, direction, word_and_letter);

    insertable = insertable && check_no_same_direction_overlaps(&position_start, &position_end, direction, crossword);

    insertable = insertable && check_other_direction_overlaps(&position_start, direction, word_and_letter, crossword);

    if insertable {
        let cross_data = CrossData {
            position: position_start,
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

    let index = direction.get_index();

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

    let z = direction.get_index();

    let start_z = position_start[z];
    let end_z = position_end[z];
    let row = position_start[(z + 1) % 2];

    clear = clear && check_row_clear(direction, row, start_z-1, end_z+1, crossword);

    clear = clear && check_row_clear(direction, row+1, start_z, end_z, crossword);

    clear = clear && check_row_clear(direction, row-1, start_z, end_z, crossword);

    return clear;
}

fn check_row_clear(direction: &Direction, row: i32, start: i32, end: i32, crossword: &Crossword) -> bool {
    let mut clear = true;

    let z = direction.get_index();
    let not_z = (z + 1) % 2;

    for word in &crossword.words {
        if let Some(cross_data) = &word.cross {
            if cross_data.direction == *direction && cross_data.position[not_z] == row {
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

fn check_other_direction_overlaps(
    position_start: &[i32; 2],
    direction: &Direction,
    word_and_letter: &WordAndLetter,
    crossword: &Crossword) -> bool {

    let mut ok = true;

    let mut position = position_start.clone();
    let index = direction.get_index();
    let other_direction = direction.change();

    position[index] -= 1;
    ok = ok && check_letter_intersections(&position, &other_direction, ' ', crossword);

    for letter in word_and_letter.word.chars() {
        position[index] += 1;
        if !ok {
            break;
        }
        ok = ok && check_letter_intersections(&position, &other_direction, letter, crossword);
    }

    position[index] += 1;
    ok = ok && check_letter_intersections(&position, &other_direction, ' ', crossword);

    return ok;
}

fn check_letter_intersections(position: &[i32; 2], direction: &Direction, letter: char, crossword: &Crossword) -> bool {
    let mut ok = true;
    let index = direction.get_index();
    let other = (index + 1) % 2;

    for word in &crossword.words {
        if let Some(cross_data) = &word.cross {
            if cross_data.direction == *direction && cross_data.position[other] == position[other] {
                let nth = position[index] - cross_data.position[index];
                if nth >= 0 && nth < word.word.len() as i32 {
                    for (i, word_letter) in word.word.chars().enumerate() {
                        if i as i32 == nth {
                            ok = ok && word_letter == letter;
                            break;
                        }
                    }
                }

                if !ok {
                    break;
                }
            }
        }
    }

    return ok;
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


