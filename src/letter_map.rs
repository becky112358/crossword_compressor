
use std::collections::HashMap;

pub struct WordAndLetter<'a> {
    pub word_index: usize,
    pub word: &'a str,
    pub letter: char,
    pub n_letters_before: usize,
    pub letter_index: usize,
    pub n_letters_after: usize,
}

pub fn get_letter_map<'a>(words: &'a Vec<&str>) -> HashMap<char, Vec<WordAndLetter<'a>>> {
    let mut letter_map: HashMap<char, Vec<WordAndLetter>> = HashMap::new();

    for word_index in 0..words.len() {
        for (letter_index, letter) in words[word_index].to_lowercase().chars().enumerate() {
            let word_and_letter = WordAndLetter {
                word_index,
                word: words[word_index],
                letter,
                n_letters_before: letter_index,
                letter_index,
                n_letters_after: words[word_index].len() - letter_index - 1,
            };

            let words_with_letter = letter_map.entry(letter).or_insert(vec![]);
            words_with_letter.push(word_and_letter);
        }
    }

    return letter_map;
}


#[cfg(test)]
#[path = "./tests_letter_map.rs"]
mod tests_letter_map;


