
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

pub fn get_letter_map<'a>(words: &'a Vec<WordUsage>) -> HashMap<char, Vec<&'a WordUsage<'a>>> {
    let mut letter_map: HashMap<char, Vec<&WordUsage>> = HashMap::new();

    for word in words {
        for letter in word.word.to_lowercase().chars() {
            let mut words_with_letter = letter_map.entry(letter).or_insert(vec![word]);
            if !words_with_letter.contains(&word) {
                words_with_letter.push(word);
            }
        }
    }

    return letter_map;
}


#[cfg(test)]
#[path = "./tests_letter_map.rs"]
mod tests_letter_map;


