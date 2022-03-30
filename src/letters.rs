use std::collections::HashMap;

pub struct WordAndLetter<'a> {
    pub word_index: usize,
    pub word: &'a str,
    pub letter: char,
    pub letter_index: usize,
    pub n_letters_after: usize,
}

pub fn letters_to_lowercase(words: &[&str]) -> Vec<String> {
    return words.iter().map(|x| x.to_lowercase()).collect();
}

pub fn letters_get_map(words: &[String]) -> HashMap<char, Vec<WordAndLetter>> {
    let mut letter_map: HashMap<char, Vec<WordAndLetter>> = HashMap::new();

    for (word_index, word) in words.iter().enumerate() {
        for (letter_index, letter) in word.chars().enumerate() {
            let word_and_letter = WordAndLetter {
                word_index,
                word,
                letter,
                letter_index,
                n_letters_after: word.len() - letter_index - 1,
            };

            let words_with_letter = letter_map.entry(letter).or_insert(vec![]);
            words_with_letter.push(word_and_letter);
        }
    }

    letter_map
}

#[cfg(test)]
#[path = "./tests_letters.rs"]
mod tests_letters;
