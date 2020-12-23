
use std::collections::HashMap;

use crate::common::WordUsage;
use crate::crossword::{Crossword, EMPTY};

#[derive(PartialEq)]
enum Direction {
    Across,
    Down,
    NotCrossable
}

pub fn compare_options<'a>(words: &mut HashMap<char, Vec<&'a mut WordUsage<'a>>>, crossword: &mut Crossword) -> Vec<Crossword> {

    let mut best_crosswords;

    for y_index in crossword.upper_edge..=crossword.lower_edge {
        for x_index in crossword.left_edge..=crossword.right_edge {
            let direction = is_crossable_letter(crossword, x_index, y_index);
            if direction != Direction::NotCrossable {
                let available_words = get_available_words(crossword.letters[x_index][y_index], words);
                if available_words.len() > 0 {
                    for mut word in available_words {
                        if insert_word(x_index, y_index, &direction, &mut word, crossword) {
                            if remaining_available_words(words) {
                                best_crosswords = compare_options(words, crossword);
                            } else {
                                compare_crosswords(crossword, &mut best_crosswords);
                            }
                            remove_word(x_index, y_index, &direction, &mut word, crossword);
                        }
                    }
                }
            }
        }
    }

    return best_crosswords;
}

fn is_crossable_letter(crossword: &Crossword, x_index: usize, y_index: usize) -> Direction {
    let direction;

    if crossword.letters[x_index][y_index] == EMPTY {
        direction = Direction::NotCrossable;
    } else if (x_index == 0 || crossword.letters[x_index-1][y_index] == EMPTY)
            && (x_index == crossword.letters.len() - 1 || crossword.letters[x_index+1][y_index] == EMPTY) {
        direction = Direction::Across;
    } else if (y_index == 0 || crossword.letters[x_index][y_index-1] == EMPTY)
            && (y_index == crossword.letters[0].len() - 1 || crossword.letters[x_index][y_index+1] == EMPTY) {
        direction = Direction::Down;
    } else {
        direction = Direction::NotCrossable;
    }

    return direction;
}

fn get_available_words<'a>(
    letter: char,
    words: &HashMap<char,
    Vec<&'a mut WordUsage<'a>>>) -> Vec<&'a mut WordUsage<'a>> {

    let mut available_words = Vec::new();

    match words.get(&letter) {
        Some(word_usages) => {
            for word in word_usages {
                if !word.currently_in_use {
                    available_words.push(*word);
                }
            }
        }
        None => (),
    }

    return available_words;
}

fn insert_word(
    x_index: usize,
    y_index: usize,
    direction: &Direction,
    word: &mut WordUsage,
    crossword: &mut Crossword) -> bool {

    // TODO FULL of duplication!!
    let mut insertable = true;

    let cross_letter = crossword.letters[x_index][y_index];
    let cross_index;

    for (index, letter) in word.word.chars().enumerate() {
        if letter == cross_letter {
            cross_index = index;
            break;
        }
    }

    let length_after = word.word.len() - cross_index - 1;
    let length_before = word.word.len() - length_after - 1;

    if *direction == Direction::Across {
        if x_index - length_before < 0 {
            insertable = false;
        } else if x_index - length_before > 0 && crossword.letters[x_index - length_before - 1][y_index] != EMPTY {
            insertable = false;
        } else if x_index + length_after > crossword.letters.len() - 1 {
            insertable = false;
        } else if x_index + length_after < crossword.letters.len() - 1
                && crossword.letters[x_index + length_after + 1][y_index] != EMPTY {
            insertable = false;
        }
    } else if *direction == Direction::Down {
        if y_index - length_before < 0 {
            insertable = false;
        } else if y_index - length_before > 0 && crossword.letters[x_index][y_index - length_before - 1] != EMPTY {
            insertable = false;
        } else if y_index + length_after > crossword.letters[0].len() - 1 {
            insertable = false;
        } else if y_index + length_after < crossword.letters[0].len() - 1
                && crossword.letters[x_index][y_index + length_after + 1] != EMPTY {
            insertable = false;
        }
    }

    if insertable && *direction == Direction::Across {
        for x in x_index - length_before..=x_index + length_after {
            if x == x_index {
                continue;
            }

            if crossword.letters[x][y_index] == EMPTY {
                if y_index > 0 && crossword.letters[x][y_index-1] != EMPTY {
                    insertable = false;
                    break;
                } else if y_index < crossword.letters[0].len() - 1 && crossword.letters[x][y_index+1] != EMPTY {
                    insertable = false;
                    break;
                }
            }
            
        }
    } else if insertable && *direction == Direction::Down {
        for y in y_index - length_before..=y_index + length_after {
            if y == y_index {
                continue;
            }

            if crossword.letters[x_index][y] == EMPTY {
                if x_index > 0 && crossword.letters[x_index-1][y] != EMPTY {
                    insertable = false;
                    break;
                } else if x_index < crossword.letters.len() - 1 && crossword.letters[x_index+1][y] != EMPTY {
                    insertable = false;
                    break;
                }
            }
            
        }
    }

    if insertable && *direction == Direction::Across {
        let mut x_index_current = x_index - length_before;
        for letter in word.word.chars() {
            if crossword.letters[x_index_current][y_index] != EMPTY
                && crossword.letters[x_index_current][y_index] != letter {
                insertable = false;
                break;
            }
            x_index_current += 1;
        }
    } else if insertable && *direction == Direction::Down {
        let mut y_index_current = y_index - length_before;
        for letter in word.word.chars() {
            if crossword.letters[x_index][y_index_current] != EMPTY
                && crossword.letters[x_index][y_index_current] != letter {
                insertable = false;
                break;
            }
            y_index_current += 1;
        }
    }

    if insertable && *direction == Direction::Across {
        let mut x_index_current = x_index - length_before;
        for letter in word.word.chars() {
            crossword.letters[x_index_current][y_index] = letter;
            x_index_current += 1;
        }
    } else if insertable && *direction == Direction::Down {
        let mut y_index_current = y_index - length_before;
        for letter in word.word.chars() {
            crossword.letters[x_index][y_index_current] = letter;
            y_index_current += 1;
        }
    }

    if insertable {
        word.currently_in_use = true;

        if *direction == Direction::Across {
            if x_index - length_before < crossword.left_edge {
                crossword.left_edge = x_index - length_before;
            }
            if x_index + length_after > crossword.right_edge {
                crossword.right_edge = x_index + length_after;
            }
        } else if *direction == Direction::Down {
            if y_index - length_before < crossword.upper_edge {
                crossword.upper_edge = y_index - length_before;
            }
            if y_index + length_after > crossword.lower_edge {
                crossword.lower_edge = y_index + length_after;
            }
        }
    }

    return insertable;
}

fn remaining_available_words(words: &HashMap<char, Vec<&mut WordUsage>>) -> bool {
    let mut remaining_available = false;

    for (_, word_usage_vec) in words {
        for word in word_usage_vec {
            if !word.currently_in_use {
                remaining_available = true;
                break;
            }
        }
    }

    return remaining_available;
}

fn compare_crosswords(crossword: &Crossword, best_crosswords: &mut Vec<Crossword>) {
    if best_crosswords.len() == 0 {
        let clone = crossword.clone();
        best_crosswords.push(clone);
    } else {
        let (current_min, current_max) = best_crosswords[0].get_min_max();
        let (new_min, new_max) = crossword.get_min_max();

        if (new_max < current_max) || (new_max == current_max && new_min < current_min) {
            let clone = crossword.clone();
            while best_crosswords.len() > 0 {       // TODO: What's happenning to memory here?
                best_crosswords.pop();
            }
            best_crosswords.push(clone);
        } else if (new_max == current_max) && new_min == current_min {
            let clone = crossword.clone();
            best_crosswords.push(clone);
        }
    }
}

fn remove_word(x_index: usize, y_index: usize, direction: &Direction, word: &mut WordUsage, crossword: &mut Crossword) {

    // TODO Duplicate!!
    // Will move this into 'get_available_words', so that word containing the relevant letter multiple times
    // are considered for each letter.
    let cross_letter = crossword.letters[x_index][y_index];
    let cross_index;

    for (index, letter) in word.word.chars().enumerate() {
        if letter == cross_letter {
            cross_index = index;
            break;
        }
    }

    // TODO FULL of duplication!!

    let length_after = word.word.len() - cross_index - 1;
    let length_before = word.word.len() - length_after - 1;

    if *direction == Direction::Across {
        for x in x_index-length_before..=x_index+length_after {
            if x == x_index {
                continue;
            } else if (y_index > 0 && crossword.letters[x][y_index-1] != EMPTY)
                || (y_index < crossword.letters[0].len() - 1 && crossword.letters[x][y_index+1] != EMPTY) {
                continue;
            } else {
                crossword.letters[x][y_index] = EMPTY;
            }
        }
    } else if *direction == Direction::Down {
        for y in y_index-length_before..=y_index+length_after {
            if y == y_index {
                continue;
            } else if (x_index > 0 && crossword.letters[x_index-1][y] != EMPTY)
                || (x_index < crossword.letters.len() - 1 && crossword.letters[x_index+1][y] != EMPTY) {
                continue;
            } else {
                crossword.letters[x_index][y] = EMPTY;
            }
        }
    }

    if *direction == Direction::Across {
        if x_index - length_before == crossword.left_edge {
            let mut found_edge = false;
            for x in crossword.left_edge..=crossword.right_edge {
                for y in crossword.upper_edge..=crossword.lower_edge {
                    if crossword.letters[x][y] != EMPTY {
                        crossword.left_edge = x;
                        found_edge = true;
                        break;
                    }
                }
                if found_edge {
                    break;
                }
            }
        }

        if x_index + length_after == crossword.right_edge {
            let mut found_edge = false;
            for x in (crossword.left_edge..=crossword.right_edge).rev() {
                for y in crossword.upper_edge..=crossword.lower_edge {
                    if crossword.letters[x][y] != EMPTY {
                        crossword.left_edge = x;
                        found_edge = true;
                        break;
                    }
                }
                if found_edge {
                    break;
                }
            }
        }
    } else if *direction == Direction::Down {
        if y_index - length_before == crossword.upper_edge {
            let mut found_edge = false;
            for y in crossword.upper_edge..=crossword.lower_edge {
                for x in crossword.left_edge..=crossword.right_edge {
                    if crossword.letters[x][y] != EMPTY {
                        crossword.upper_edge = y;
                        found_edge = true;
                        break;
                    }
                }
                if found_edge {
                    break;
                }
            }
        }

        if y_index + length_after == crossword.lower_edge {
            let mut found_edge = false;
            for y in (crossword.upper_edge..=crossword.lower_edge).rev() {
                for x in crossword.left_edge..=crossword.right_edge {
                    if crossword.letters[x][y] != EMPTY {
                        crossword.lower_edge = y;
                        found_edge = true;
                        break;
                    }
                }
                if found_edge {
                    break;
                }
            }
        }
    }
}

