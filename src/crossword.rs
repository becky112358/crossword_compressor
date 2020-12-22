
use crate::common::Crossword;

pub fn create_crossword(words: &Vec<&str>, start_word: &str) -> Crossword {

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

    let mut crossword = Crossword {
        left_edge: sum_of_longest_words,
        upper_edge: sum_of_longest_words,
        right_edge: sum_of_longest_words + start_word.len() - 1,
        lower_edge: sum_of_longest_words,
        letters: vec![vec!['#'; sum_of_longest_words * 2 + 1]; sum_of_longest_words * 2 + start_word.len()],
    };

    let mut x_index = sum_of_longest_words;
    for c in start_word.chars() {
        crossword.letters[x_index][sum_of_longest_words] = c;
        x_index += 1;
    }

    return crossword;
}


