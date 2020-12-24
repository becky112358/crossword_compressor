
use std::collections::HashMap;

use crate::crossword::{Crossword, EMPTY, X, Y, MIN, MAX};
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

struct CrossData<'a> {
    x_index: usize,
    y_index: usize,
    direction: &'a Direction,
    word_and_letter: &'a WordAndLetter<'a>,
}

pub fn compare_options(
    letter_map: &HashMap<char, Vec<WordAndLetter>>,
    mut words_in_crossword: &mut Vec<bool>,
    crossword: &mut Crossword,
    best_crosswords: &mut Vec<Crossword>) {

    for x_index in crossword.edges[X][MIN]..=crossword.edges[X][MAX] {
        for y_index in crossword.edges[Y][MIN]..=crossword.edges[Y][MAX] {
            let direction = is_crossable_letter(crossword, x_index, y_index);
            let crossable_words = letter_map.get(&crossword.letters[x_index][y_index]);
            match crossable_words {
                Some(crossable_words) => if direction != Direction::NotCrossable {
                    for word_and_letter in crossable_words {
                        let cross_data = CrossData {
                            x_index,
                            y_index,
                            direction: &direction,
                            word_and_letter: &word_and_letter };

                        if insert_word(&cross_data, &mut words_in_crossword, crossword) {
                            let crossword_status = compare_crosswords(crossword, best_crosswords);
                            if crossword_status == Comparison::Worse {
                            } else if words_in_crossword.contains(&false) {
                                compare_options(letter_map, words_in_crossword, crossword, best_crosswords);
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

fn insert_word(cross_data: &CrossData, words_in_crossword: &mut Vec<bool>, mut crossword: &mut Crossword) -> bool {

    // TODO FULL of duplication!!
    let mut insertable = !words_in_crossword[cross_data.word_and_letter.word_index];

    insertable = insertable && can_fit_word_in_crossword(cross_data, crossword);

    insertable = insertable && are_endpoints_clear(cross_data, crossword);

    if insertable && *cross_data.direction == Direction::Across {
        for x in cross_data.x_index - cross_data.word_and_letter.n_letters_before..=cross_data.x_index + cross_data.word_and_letter.n_letters_after {
            if x == cross_data.x_index {
                continue;
            }

            if crossword.letters[x][cross_data.y_index] == EMPTY {
                if cross_data.y_index > 0 && crossword.letters[x][cross_data.y_index-1] != EMPTY {
                    insertable = false;
                    break;
                } else if cross_data.y_index < crossword.letters[0].len() - 1 && crossword.letters[x][cross_data.y_index+1] != EMPTY {
                    insertable = false;
                    break;
                }
            }
            
        }
    } else if insertable && *cross_data.direction == Direction::Down {
        for y in cross_data.y_index - cross_data.word_and_letter.n_letters_before..=cross_data.y_index + cross_data.word_and_letter.n_letters_after {
            if y == cross_data.y_index {
                continue;
            }

            if crossword.letters[cross_data.x_index][y] == EMPTY {
                if cross_data.x_index > 0 && crossword.letters[cross_data.x_index-1][y] != EMPTY {
                    insertable = false;
                    break;
                } else if cross_data.x_index < crossword.letters.len() - 1 && crossword.letters[cross_data.x_index+1][y] != EMPTY {
                    insertable = false;
                    break;
                }
            }
            
        }
    }

    insertable = insertable && all_letters_in_path_match(cross_data, crossword);

    if insertable {
        write_word(cross_data, &mut crossword);
    }

    if insertable {
        words_in_crossword[cross_data.word_and_letter.word_index] = true;

        if *cross_data.direction == Direction::Across {
            if cross_data.x_index - cross_data.word_and_letter.n_letters_before < crossword.edges[X][MIN] {
                crossword.edges[X][MIN] = cross_data.x_index - cross_data.word_and_letter.n_letters_before;
            }
            if cross_data.x_index + cross_data.word_and_letter.n_letters_after > crossword.edges[X][MAX] {
                crossword.edges[X][MAX] = cross_data.x_index + cross_data.word_and_letter.n_letters_after;
            }
        } else if *cross_data.direction == Direction::Down {
            if cross_data.y_index - cross_data.word_and_letter.n_letters_before < crossword.edges[Y][MIN] {
                crossword.edges[Y][MIN] = cross_data.y_index - cross_data.word_and_letter.n_letters_before;
            }
            if cross_data.y_index + cross_data.word_and_letter.n_letters_after > crossword.edges[Y][MAX] {
                crossword.edges[Y][MAX] = cross_data.y_index + cross_data.word_and_letter.n_letters_after;
            }
        }
    }

    return insertable;
}

fn can_fit_word_in_crossword(cross_data: &CrossData, crossword: &Crossword) -> bool {
    let can_fit;
    let index = if *cross_data.direction == Direction::Across { cross_data.x_index } else { cross_data.y_index };
    let crossword_width = if *cross_data.direction == Direction::Across { crossword.letters.len() } 
                                                                   else { crossword.letters[0].len() };

    if cross_data.word_and_letter.n_letters_before > index {
        can_fit = false;
    } else if index + cross_data.word_and_letter.n_letters_after > crossword_width - 1 {
        can_fit = false;
    } else {
        can_fit = true;
    }

    return can_fit;
}

fn are_endpoints_clear(cross_data: &CrossData, crossword: &Crossword) -> bool {
    let mut clear = true;

    let endpoint_before = cross_data.word_and_letter.n_letters_before + 1;
    let endpoint_after = cross_data.word_and_letter.n_letters_after + 1;

    if *cross_data.direction == Direction::Across {
        if cross_data.x_index > cross_data.word_and_letter.n_letters_before
        && crossword.letters[cross_data.x_index - endpoint_before][cross_data.y_index] != EMPTY {
            clear = false;
        } else if cross_data.x_index + cross_data.word_and_letter.n_letters_after < crossword.letters.len() - 1
               && crossword.letters[cross_data.x_index + endpoint_after][cross_data.y_index] != EMPTY {
            clear = false;
        }
    } else if *cross_data.direction == Direction::Down {
        if cross_data.y_index > cross_data.word_and_letter.n_letters_before
        && crossword.letters[cross_data.x_index][cross_data.y_index - endpoint_before] != EMPTY {
            clear = false;
        } else if cross_data.y_index + cross_data.word_and_letter.n_letters_after < crossword.letters[0].len() - 1
               && crossword.letters[cross_data.x_index][cross_data.y_index + endpoint_after] != EMPTY {
            clear = false;
        }
    }

    return clear;
}

fn all_letters_in_path_match(cross_data: &CrossData, crossword: &Crossword) -> bool {
    let mut all_letters_match = true;

    let (mut x, mut y) = get_x_y_start(cross_data);

    for letter in cross_data.word_and_letter.word.chars() {
        if crossword.letters[x][y] != EMPTY && crossword.letters[x][y] != letter {
            all_letters_match = false;
            break;
        }
        get_x_y_next(cross_data, &mut x, &mut y);
    }

    return all_letters_match;
}

fn write_word(cross_data: &CrossData, crossword: &mut Crossword) {
    let (mut x, mut y) = get_x_y_start(cross_data);

    for letter in cross_data.word_and_letter.word.chars() {
        crossword.letters[x][y] = letter;
        get_x_y_next(cross_data, &mut x, &mut y);
    }
}

fn get_x_y_start(cross_data: &CrossData) -> (usize, usize) {
    let mut x = cross_data.x_index;
    let mut y = cross_data.y_index;

    match *cross_data.direction {
        Direction::Across => x -= cross_data.word_and_letter.n_letters_before,
        Direction::Down => y -= cross_data.word_and_letter.n_letters_before,
        _ => (),
    }

    return (x, y);
}

fn get_x_y_next(cross_data: &CrossData, x: &mut usize, y: &mut usize) {
    match *cross_data.direction {
        Direction::Across => *x += 1,
        Direction::Down => *y += 1,
        _ => (),
    }
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
        if x_index - word_and_letter.n_letters_before == crossword.edges[X][MIN] {
            let mut found_edge = false;
            for x in crossword.edges[X][MIN]..=crossword.edges[X][MAX] {
                for y in crossword.edges[Y][MIN]..=crossword.edges[Y][MAX] {
                    if crossword.letters[x][y] != EMPTY {
                        crossword.edges[X][MIN] = x;
                        found_edge = true;
                        break;
                    }
                }
                if found_edge {
                    break;
                }
            }
        }

        if x_index + word_and_letter.n_letters_after == crossword.edges[X][MAX] {
            let mut found_edge = false;
            for x in (crossword.edges[X][MIN]..=crossword.edges[X][MAX]).rev() {
                for y in crossword.edges[Y][MIN]..=crossword.edges[Y][MAX] {
                    if crossword.letters[x][y] != EMPTY {
                        crossword.edges[X][MAX] = x;
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
        if y_index - word_and_letter.n_letters_before == crossword.edges[Y][MIN] {
            let mut found_edge = false;
            for y in crossword.edges[Y][MIN]..=crossword.edges[Y][MAX] {
                for x in crossword.edges[X][MIN]..=crossword.edges[X][MAX] {
                    if crossword.letters[x][y] != EMPTY {
                        crossword.edges[Y][MIN] = y;
                        found_edge = true;
                        break;
                    }
                }
                if found_edge {
                    break;
                }
            }
        }

        if y_index + word_and_letter.n_letters_after == crossword.edges[Y][MAX] {
            let mut found_edge = false;
            for y in (crossword.edges[Y][MIN]..=crossword.edges[Y][MAX]).rev() {
                for x in crossword.edges[X][MIN]..=crossword.edges[X][MAX] {
                    if crossword.letters[x][y] != EMPTY {
                        crossword.edges[Y][MAX] = y;
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

