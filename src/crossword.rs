
pub const EMPTY: char = ' ';

pub const X: usize = 0;
pub const Y: usize = 1;
pub const MIN: usize = 0;
pub const MAX: usize = 1;

#[derive(Clone)]
pub struct Crossword {
    pub edges: [[usize; 2]; 2],
    pub letters: Vec<Vec<char>>,
}

impl PartialEq for Crossword {
    fn eq(&self, other: &Self) -> bool {
        let mut equal = true;

        let x_width = self.edges[X][MAX] - self.edges[X][MIN] + 1;
        let y_width = self.edges[Y][MAX] - self.edges[Y][MIN] + 1;

        if other.edges[X][MAX] - other.edges[X][MIN] + 1 != x_width {
            equal = false;
        } else if other.edges[Y][MAX] - other.edges[Y][MIN] + 1 != y_width {
            equal = false;
        } else {
            for x in 0..x_width {
                for y in 0..y_width {
                    let x_self = offset(self, X, x);
                    let y_self = offset(self, Y, y);
                    let x_other = offset(other, X, x);
                    let y_other = offset(other, Y, y);
                    if self.letters[x_self][y_self] != other.letters[x_other][y_other] {
                        equal = false;
                        break;
                    }
                }
            }
        }

        return equal;
    }
}

fn offset(crossword: &Crossword, index: usize, increment: usize) -> usize {
    return crossword.edges[index][MIN] + increment;
}

impl Crossword {
    pub fn get_min_max(&self) -> (usize, usize) {
        let x_len = self.edges[X][MAX] - self.edges[X][MIN] + 1;
        let y_len = self.edges[Y][MAX] - self.edges[Y][MIN] + 1;

        let min;
        let max;

        if x_len < y_len {
            min = x_len;
            max = y_len;
        } else {
            min = y_len;
            max = x_len;
        }

        return (min, max);
    }

    pub fn clone_shrink(&self) -> Crossword {
        let x_size = self.edges[X][MAX] - self.edges[X][MIN] + 1;
        let y_size = self.edges[Y][MAX] - self.edges[Y][MIN] + 1;

        let mut other = Crossword {
            edges: [[0,x_size-1], [0,y_size-1]],
            letters: vec![vec![EMPTY; y_size]; x_size],
        };

        for x_index in self.edges[X][MIN]..=self.edges[X][MAX] {
            for y_index in self.edges[Y][MIN]..=self.edges[Y][MAX] {
                other.letters[x_index-self.edges[X][MIN]][y_index-self.edges[Y][MIN]] = self.letters[x_index][y_index];
            }
        }

        return other;
    }

    pub fn print(&self) {
        for y in self.edges[Y][MIN]..=self.edges[Y][MAX] {
            for x in self.edges[X][MIN]..=self.edges[X][MAX] {
                print!("{}", self.letters[x][y]);
            }
            println!();
        }
    }
}

pub fn initialise_crossword(words: &Vec<&str>, start_word: &str) -> Crossword {
    let sum_of_longest_words = get_sum_of_longest_words(words);
    let square_size = sum_of_longest_words * 2 + start_word.len();

    let mut crossword = Crossword {
        edges: [[sum_of_longest_words, sum_of_longest_words + start_word.len() - 1],
                [sum_of_longest_words, sum_of_longest_words]],
        letters: vec![vec![EMPTY; square_size]; square_size],
    };

    let mut x_index = sum_of_longest_words;
    for c in start_word.chars() {
        crossword.letters[x_index][sum_of_longest_words] = c;
        x_index += 1;
    }

    return crossword;
}

fn get_sum_of_longest_words(words: &Vec<&str>) -> usize {

    let n_words = words.len();

    let mut previous_max = usize::MAX;
    let mut current_max = 0;
    let mut n_words_of_current_max = 0;
    let mut n_words_summed = 0;
    let mut sum_of_longest_words = 0;

    while n_words_summed * 2 < n_words {
        for word in words {
            if word.len() > current_max && word.len() < previous_max {
                current_max = word.len();
                n_words_of_current_max = 0;
            } else if word.len() == current_max {
                n_words_of_current_max += 1;
            }
        }

        sum_of_longest_words = n_words_of_current_max * current_max;
        n_words_summed += n_words_of_current_max;
        previous_max = current_max;
    }

    return sum_of_longest_words;
}

