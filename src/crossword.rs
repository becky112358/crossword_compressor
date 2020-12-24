
pub const EMPTY: char = ' ';

#[derive(Clone)]
pub struct Crossword {
    pub right_edge: usize,
    pub upper_edge: usize,
    pub left_edge: usize,
    pub lower_edge: usize,
    pub letters: Vec<Vec<char>>,
}

impl Crossword {
    pub fn get_min_max(&self) -> (usize, usize) {
        let x_len = self.right_edge - self.left_edge + 1;
        let y_len = self.lower_edge - self.upper_edge + 1;

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
        let x_size = self.right_edge - self.left_edge + 1;
        let y_size = self.lower_edge - self.upper_edge + 1;

        let mut other = Crossword {
            right_edge: x_size - 1,
            upper_edge: 0,
            left_edge: 0,
            lower_edge: y_size - 1,
            letters: vec![vec![EMPTY; y_size]; x_size],
        };

        for x_index in self.left_edge..=self.right_edge {
            for y_index in self.upper_edge..=self.lower_edge {
                other.letters[x_index-self.left_edge][y_index-self.upper_edge] = self.letters[x_index][y_index];
            }
        }

        return other;
    }

    pub fn print(&self) {
        for y in self.upper_edge..=self.lower_edge {
            for x in self.left_edge..=self.right_edge {
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
        left_edge: sum_of_longest_words,
        upper_edge: sum_of_longest_words,
        right_edge: sum_of_longest_words + start_word.len() - 1,
        lower_edge: sum_of_longest_words,
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

