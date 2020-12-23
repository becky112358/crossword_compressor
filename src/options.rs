
use std::collections::HashMap;

use crate::crossword::{Crossword, EMPTY};
use crate::letter_map::WordAndLetter;

#[derive(PartialEq)]
enum Direction {
    Across,
    Down,
    NotCrossable
}

#[derive(PartialEq)]
enum Comparison {
    First,
    Better,
    AsGood,
    Worse,
}

pub fn compare_options(
    words: &Vec<&str>,
    letter_map: &HashMap<char, Vec<WordAndLetter>>,
    mut words_in_crossword: &mut Vec<bool>,
    crossword: &mut Crossword,
    best_crosswords: &mut Vec<Crossword>) {

    for y_index in crossword.upper_edge..=crossword.lower_edge {
        for x_index in crossword.left_edge..=crossword.right_edge {
            let direction = is_crossable_letter(crossword, x_index, y_index);
            if direction != Direction::NotCrossable {
                let crossable_words = letter_map.get(&crossword.letters[x_index][y_index]);
                match crossable_words {
                    Some(crossable_words) => {
                        for word_and_letter in crossable_words {
                            if insert_word(x_index, y_index, &direction, &word_and_letter, &mut words_in_crossword, crossword) {
                                let crossword_status = compare_crosswords(crossword, best_crosswords);
                                if crossword_status == Comparison::Worse {
                                } else if words_in_crossword.contains(&false) {
                                    compare_options(words, letter_map, words_in_crossword, crossword, best_crosswords);
                                } else {
                                    add_crossword(crossword_status, crossword, best_crosswords);
                                }
                                remove_word(x_index, y_index, &direction, &word_and_letter, &mut words_in_crossword, crossword);
                            }
                        }
                    },
                    None => (),
                }
            }
        }
    }
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

fn insert_word(
    x_index: usize,
    y_index: usize,
    direction: &Direction,
    word_and_letter: &WordAndLetter,
    words_in_crossword: &mut Vec<bool>,
    crossword: &mut Crossword) -> bool {

    // TODO FULL of duplication!!
    let mut insertable = !words_in_crossword[word_and_letter.word_index];

    if insertable && *direction == Direction::Across {
        if x_index - word_and_letter.n_letters_before < 0 {
            insertable = false;
        } else if x_index - word_and_letter.n_letters_before > 0
                && crossword.letters[x_index - word_and_letter.n_letters_before - 1][y_index] != EMPTY {
            insertable = false;
        } else if x_index + word_and_letter.n_letters_after > crossword.letters.len() - 1 {
            insertable = false;
        } else if x_index + word_and_letter.n_letters_after < crossword.letters.len() - 1
                && crossword.letters[x_index + word_and_letter.n_letters_after + 1][y_index] != EMPTY {
            insertable = false;
        }
    } else if insertable && *direction == Direction::Down {
        if y_index - word_and_letter.n_letters_before < 0 {
            insertable = false;
        } else if y_index - word_and_letter.n_letters_before > 0
            && crossword.letters[x_index][y_index - word_and_letter.n_letters_before - 1] != EMPTY {
            insertable = false;
        } else if y_index + word_and_letter.n_letters_after > crossword.letters[0].len() - 1 {
            insertable = false;
        } else if y_index + word_and_letter.n_letters_after < crossword.letters[0].len() - 1
                && crossword.letters[x_index][y_index + word_and_letter.n_letters_after + 1] != EMPTY {
            insertable = false;
        }
    }

    if insertable && *direction == Direction::Across {
        for x in x_index - word_and_letter.n_letters_before..=x_index + word_and_letter.n_letters_after {
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
        for y in y_index - word_and_letter.n_letters_before..=y_index + word_and_letter.n_letters_after {
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
        let mut x_index_current = x_index - word_and_letter.n_letters_before;
        for letter in word_and_letter.word.chars() {
            if crossword.letters[x_index_current][y_index] != EMPTY
                && crossword.letters[x_index_current][y_index] != letter {
                insertable = false;
                break;
            }
            x_index_current += 1;
        }
    } else if insertable && *direction == Direction::Down {
        let mut y_index_current = y_index - word_and_letter.n_letters_before;
        for letter in word_and_letter.word.chars() {
            if crossword.letters[x_index][y_index_current] != EMPTY
                && crossword.letters[x_index][y_index_current] != letter {
                insertable = false;
                break;
            }
            y_index_current += 1;
        }
    }

    if insertable && *direction == Direction::Across {
        let mut x_index_current = x_index - word_and_letter.n_letters_before;
        for letter in word_and_letter.word.chars() {
            crossword.letters[x_index_current][y_index] = letter;
            x_index_current += 1;
        }
    } else if insertable && *direction == Direction::Down {
        let mut y_index_current = y_index - word_and_letter.n_letters_before;
        for letter in word_and_letter.word.chars() {
            crossword.letters[x_index][y_index_current] = letter;
            y_index_current += 1;
        }
    }

    if insertable {
        words_in_crossword[word_and_letter.word_index] = true;

        if *direction == Direction::Across {
            if x_index - word_and_letter.n_letters_before < crossword.left_edge {
                crossword.left_edge = x_index - word_and_letter.n_letters_before;
            }
            if x_index + word_and_letter.n_letters_after > crossword.right_edge {
                crossword.right_edge = x_index + word_and_letter.n_letters_after;
            }
        } else if *direction == Direction::Down {
            if y_index - word_and_letter.n_letters_before < crossword.upper_edge {
                crossword.upper_edge = y_index - word_and_letter.n_letters_before;
            }
            if y_index + word_and_letter.n_letters_after > crossword.lower_edge {
                crossword.lower_edge = y_index + word_and_letter.n_letters_after;
            }
        }
    }

    return insertable;
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

fn add_crossword(comparison: Comparison, crossword: &Crossword, best_crosswords: &mut Vec<Crossword>) {
    match comparison {
        Comparison::Better => {
            while best_crosswords.len() > 0 {
                best_crosswords.pop();
            }
        }
        _ => (),
    }

    match comparison {
        Comparison::First | Comparison::Better | Comparison::AsGood => best_crosswords.push(crossword.clone_shrink()),
        Comparison::Worse => (),
    }
}

fn remove_word(
    x_index: usize,
    y_index: usize,
    direction: &Direction,
    word_and_letter: &WordAndLetter,
    words_in_crossword: &mut Vec<bool>,
    crossword: &mut Crossword) {

    // TODO FULL of duplication!!

    if *direction == Direction::Across {
        for x in x_index-word_and_letter.n_letters_before..=x_index+word_and_letter.n_letters_after {
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
        for y in y_index-word_and_letter.n_letters_before..=y_index+word_and_letter.n_letters_after {
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
        if x_index - word_and_letter.n_letters_before == crossword.left_edge {
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

        if x_index + word_and_letter.n_letters_after == crossword.right_edge {
            let mut found_edge = false;
            for x in (crossword.left_edge..=crossword.right_edge).rev() {
                for y in crossword.upper_edge..=crossword.lower_edge {
                    if crossword.letters[x][y] != EMPTY {
                        crossword.right_edge = x;
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
        if y_index - word_and_letter.n_letters_before == crossword.upper_edge {
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

        if y_index + word_and_letter.n_letters_after == crossword.lower_edge {
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

    words_in_crossword[word_and_letter.word_index] = false;
}

