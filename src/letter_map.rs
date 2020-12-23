
use std::collections::HashMap;

use crate::common::WordUsage;

pub fn create_word_usages<'a>(words: &'a Vec<&str>) -> Vec<WordUsage<'a>> {
    let mut collection = Vec::with_capacity(words.len());

    for word in words {

        let word_usage = WordUsage {
            word: word,
            currently_in_use: false,
        };

        collection.push(word_usage);
    }

    return collection;
}

pub fn get_letter_map<'a>(words: &'a mut Vec<WordUsage<'a>>) -> HashMap<char, Vec<&'a mut WordUsage<'a>>> {
    let mut letter_map: HashMap<char, Vec<&mut WordUsage>> = HashMap::new();

    for i in 0..words.len() {
        let word_string = &words[i].word.to_lowercase();

        for letter in word_string.chars() {
            let mut words_with_letter = letter_map.entry(letter).or_insert(vec![&mut words[i]]);
            if !words_with_letter.contains(&&mut words[i]) {
                words_with_letter.push(&mut words[i]);
            }
        }

    }

    return letter_map;
}


#[cfg(test)]
#[path = "./tests_letter_map.rs"]
mod tests_letter_map;


