
use std::collections::HashMap;

use crate::crossword::{Crossword, EMPTY, X, Y, MIN, MAX};
use crate::letter_map::WordAndLetter;

#[derive(PartialEq)]
enum Direction {
    Across,
    Down,
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

    // todo Write an iterator on crossword that returns (x_index, y_index, direction)
    for x_index in crossword.edges[X][MIN]..=crossword.edges[X][MAX] {
        for y_index in crossword.edges[Y][MIN]..=crossword.edges[Y][MAX] {
            if let Some(direction) = is_crossable_letter(crossword, x_index, y_index) {
                if let Some(crossable_words) = letter_map.get(&crossword.letters[x_index][y_index]) {
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
                            remove_word(&cross_data, &mut words_in_crossword, crossword);
                        }
                    }
                }
            }
        }
    }
}

fn is_crossable_letter(crossword: &Crossword, x_index: usize, y_index: usize) -> Option<Direction> {
    let direction;

    if crossword.letters[x_index][y_index] == EMPTY {
        direction = None;
    } else if (x_index == 0 || crossword.letters[x_index-1][y_index] == EMPTY)
            && (x_index == crossword.letters.len() - 1 || crossword.letters[x_index+1][y_index] == EMPTY) {
        direction = Some(Direction::Across);
    } else if (y_index == 0 || crossword.letters[x_index][y_index-1] == EMPTY)
            && (y_index == crossword.letters[0].len() - 1 || crossword.letters[x_index][y_index+1] == EMPTY) {
        direction = Some(Direction::Down);
    } else {
        direction = None;
    }

    return direction;
}

fn insert_word(cross_data: &CrossData, words_in_crossword: &mut Vec<bool>, mut crossword: &mut Crossword) -> bool {

    let mut insertable = !words_in_crossword[cross_data.word_and_letter.word_index];

    insertable = insertable && check_word_fits_in_crossword(cross_data, crossword);

    insertable = insertable && check_endpoints(cross_data, crossword);

    insertable = insertable && check_sides_clear_except_at_crosspoints(cross_data, crossword);

    insertable = insertable && check_all_letters_in_path_match(cross_data, crossword);

    if insertable {
        write_word(cross_data, &mut crossword);
        words_in_crossword[cross_data.word_and_letter.word_index] = true;
        update_edges_for_insert(cross_data, &mut crossword);
    }

    return insertable;
}

fn check_word_fits_in_crossword(cross_data: &CrossData, crossword: &Crossword) -> bool {
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

fn check_endpoints(cross_data: &CrossData, crossword: &Crossword) -> bool {
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

fn check_sides_clear_except_at_crosspoints(cross_data: &CrossData, crossword: &Crossword) -> bool {
    let mut clear = true;

    if *cross_data.direction == Direction::Across {
        let (x_lower, x_upper) = get_x_lower_upper(cross_data);
        let y = cross_data.y_index;
        for x in x_lower..=x_upper {
            if x == cross_data.x_index {
                continue;
            }

            if crossword.letters[x][y] == EMPTY {
                if y > 0 && crossword.letters[x][y-1] != EMPTY {
                    clear = false;
                    break;
                } else if y < crossword.letters[0].len() - 1 && crossword.letters[x][y+1] != EMPTY {
                    clear = false;
                    break;
                }
            }
            
        }
    } else if *cross_data.direction == Direction::Down {
        let x = cross_data.x_index;
        let (y_lower, y_upper) = get_y_lower_upper(cross_data);
        for y in y_lower..=y_upper {
            if y == cross_data.y_index {
                continue;
            }

            if crossword.letters[x][y] == EMPTY {
                if x > 0 && crossword.letters[x-1][y] != EMPTY {
                    clear = false;
                    break;
                } else if x < crossword.letters.len() - 1 && crossword.letters[x+1][y] != EMPTY {
                    clear = false;
                    break;
                }
            }
            
        }
    }

    return clear;
}

fn check_all_letters_in_path_match(cross_data: &CrossData, crossword: &Crossword) -> bool {
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
    }

    return (x, y);
}

fn get_x_y_next(cross_data: &CrossData, x: &mut usize, y: &mut usize) {
    match *cross_data.direction {
        Direction::Across => *x += 1,
        Direction::Down => *y += 1,
    }
}

fn update_edges_for_insert(cross_data: &CrossData, crossword: &mut Crossword) {
    let letter_index = if *cross_data.direction == Direction::Across { cross_data.x_index } else { cross_data.y_index };
    let edge_index = if *cross_data.direction == Direction::Across { X } else { Y };

    crossword.edges[edge_index][MIN] = crossword.edges[edge_index][MIN].min(
                                       letter_index - cross_data.word_and_letter.n_letters_before);
    crossword.edges[edge_index][MAX] = crossword.edges[edge_index][MAX].max(
                                       letter_index + cross_data.word_and_letter.n_letters_after);
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

fn remove_word(cross_data: &CrossData, words_in_crossword: &mut Vec<bool>, crossword: &mut Crossword) {
    remove_letters(cross_data, crossword);
    update_edges_for_remove(cross_data, crossword);
    words_in_crossword[cross_data.word_and_letter.word_index] = false;
}

fn remove_letters(cross_data: &CrossData, crossword: &mut Crossword) {
    if *cross_data.direction == Direction::Across {
        let (x_lower, x_upper) = get_x_lower_upper(cross_data);
        let y = cross_data.y_index;
        for x in x_lower..=x_upper {
            if x == cross_data.x_index {
                continue;
            } else if (y > 0 && crossword.letters[x][y-1] != EMPTY)
                   || (y < crossword.letters[0].len() - 1 && crossword.letters[x][y+1] != EMPTY) {
                continue;
            } else {
                crossword.letters[x][y] = EMPTY;
            }
        }
    } else if *cross_data.direction == Direction::Down {
        let x = cross_data.x_index;
        let (y_lower, y_upper) = get_y_lower_upper(cross_data);
        for y in y_lower..=y_upper {
            if y == cross_data.y_index {
                continue;
            } else if (x > 0 && crossword.letters[x-1][y] != EMPTY)
                   || (x < crossword.letters.len() - 1 && crossword.letters[x+1][y] != EMPTY) {
                continue;
            } else {
                crossword.letters[x][y] = EMPTY;
            }
        }
    }
}

fn get_x_lower_upper(cross_data: &CrossData) -> (usize, usize) {
    let x_lower = cross_data.x_index - cross_data.word_and_letter.n_letters_before;
    let x_upper = cross_data.x_index + cross_data.word_and_letter.n_letters_after;
    return (x_lower, x_upper);
}

fn get_y_lower_upper(cross_data: &CrossData) -> (usize, usize) {
    let y_lower = cross_data.y_index - cross_data.word_and_letter.n_letters_before;
    let y_upper = cross_data.y_index + cross_data.word_and_letter.n_letters_after;
    return (y_lower, y_upper);
}

fn update_edges_for_remove(cross_data: &CrossData, crossword: &mut Crossword) {
    // todo Reduce the duplication
    if *cross_data.direction == Direction::Across {
        if cross_data.x_index - cross_data.word_and_letter.n_letters_before == crossword.edges[X][MIN] {
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

        if cross_data.x_index + cross_data.word_and_letter.n_letters_after == crossword.edges[X][MAX] {
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
    } else if *cross_data.direction == Direction::Down {
        if cross_data.y_index - cross_data.word_and_letter.n_letters_before == crossword.edges[Y][MIN] {
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

        if cross_data.y_index + cross_data.word_and_letter.n_letters_after == crossword.edges[Y][MAX] {
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
}


